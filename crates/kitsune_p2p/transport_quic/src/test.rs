#[cfg(test)]
mod tests {
    use crate::*;
    use futures::stream::StreamExt;
    use kitsune_p2p_types::dependencies::spawn_pressure;
    use kitsune_p2p_types::metrics;
    use kitsune_p2p_types::metrics::metric_task_warn_limit;
    use kitsune_p2p_types::metrics::throughput;
    use kitsune_p2p_types::transport::*;

    #[tokio::test(threaded_scheduler)]
    async fn test_message() {
        let (listener1, _events1) = spawn_transport_listener_quic(
            ConfigListenerQuic::default().set_override_host(Some("127.0.0.1")),
        )
        .await
        .unwrap();

        let bound1 = listener1.bound_url().await.unwrap();
        assert_eq!("127.0.0.1", bound1.host_str().unwrap());
        println!("listener1 bound to: {}", bound1);

        let (listener2, mut events2) = spawn_transport_listener_quic(ConfigListenerQuic::default())
            .await
            .unwrap();

        metric_task_warn_limit(spawn_pressure::spawn_limit!(1000), async move {
            while let Some(evt) = events2.next().await {
                match evt {
                    TransportEvent::IncomingChannel(url, mut write, read) => {
                        println!("events2 incoming connection: {}", url,);
                        let data = read.read_to_end().await;
                        println!("message from {} : {}", url, String::from_utf8_lossy(&data),);
                        let data = format!("echo: {}", String::from_utf8_lossy(&data)).into_bytes();
                        write.write_and_close(data).await?;
                    }
                }
            }
            TransportResult::Ok(())
        });

        let bound2 = listener2.bound_url().await.unwrap();
        println!("listener2 bound to: {}", bound2);

        let resp = listener1.request(bound2, b"hello".to_vec()).await.unwrap();

        println!("got resp: {}", String::from_utf8_lossy(&resp));

        assert_eq!("echo: hello", &String::from_utf8_lossy(&resp));
    }

    #[tokio::test(threaded_scheduler)]
    async fn test_large_message() {
        let (listener1, _events1) = spawn_transport_listener_quic(
            ConfigListenerQuic::default().set_override_host(Some("127.0.0.1")),
        )
        .await
        .unwrap();

        let bound1 = listener1.bound_url().await.unwrap();
        assert_eq!("127.0.0.1", bound1.host_str().unwrap());
        println!("listener1 bound to: {}", bound1);

        let (listener2, mut events2) = spawn_transport_listener_quic(ConfigListenerQuic::default())
            .await
            .unwrap();

        metric_task_warn_limit(spawn_pressure::spawn_limit!(1000), async move {
            while let Some(evt) = events2.next().await {
                match evt {
                    TransportEvent::IncomingChannel(_url, mut write, read) => {
                        let data = read.read_to_end().await;
                        let data = format!("echo: {}", String::from_utf8_lossy(&data)).into_bytes();
                        write.write_and_close(data).await?;
                    }
                }
            }
            TransportResult::Ok(())
        });

        let bound2 = listener2.bound_url().await.unwrap();

        let large_msg = std::iter::repeat(b"a"[0]).take(70_000).collect::<Vec<_>>();
        let resp = listener1.request(bound2, large_msg.clone()).await.unwrap();

        assert_eq!(
            format!("echo: {}", String::from_utf8_lossy(&large_msg)),
            String::from_utf8_lossy(&resp)
        );
        assert_eq!(resp.len(), 70_006);
    }

    #[tokio::test(threaded_scheduler)]
    async fn pool_tp() -> TransportResult<()> {
        use kitsune_p2p_types::transport_pool::*;
        let (c1, p1, e1) = spawn_transport_pool().await?;
        let (sub1, sube1) = spawn_transport_listener_quic(ConfigListenerQuic::default())
            .await
            .unwrap();
        let suburl1 = sub1.bound_url().await?;
        c1.push_sub_transport(sub1, sube1).await?;
        test_receiver(e1);

        let _ = p1.bound_url().await?;

        let bytes = 100;
        let p1 = &p1;
        throughput(bytes, 10, || {
            let suburl1 = suburl1.clone();
            async move {
                let _ = p1
                    .request(suburl1, vec![0u8; bytes as usize])
                    .await
                    .unwrap();
            }
        })
        .await;

        Ok(())
    }

    fn test_receiver(mut recv: TransportEventReceiver) {
        metrics::metric_task_warn_limit(spawn_pressure::spawn_limit!(500), async move {
            while let Some(evt) = recv.next().await {
                match evt {
                    TransportEvent::IncomingChannel(url, mut write, read) => {
                        let data = read.read_to_end().await;
                        let data = format!("echo({}): {}", url, String::from_utf8_lossy(&data),);
                        write.write_and_close(data.into_bytes()).await?;
                    }
                }
            }
            TransportResult::Ok(())
        });
    }
}
