use anyhow::Result;
use eventsource_stream::Eventsource;
use futures::channel::oneshot;
use futures::stream::StreamExt;
use reqwest::Client;
use std::{ops::DerefMut, sync::Arc};
use stream_cancel::{StreamExt as CancellableStreamExt, Trigger, Tripwire};
use tokio::sync::Mutex;

type StreamCallback = Box<dyn Fn(String) -> Result<String>>;
//pub struct OpenAiStream<T>
//where
//    T: FnMut(String) -> Result<String>,
pub struct OpenAiStream {
    //pub callback: Option<Box<StreamCallback>>,
    trigger: Arc<Mutex<Option<Box<Trigger>>>>,
    tripwire: Tripwire,
}

impl OpenAiStream
//impl<T> OpenAiStream<T>
//where
//    T: FnMut(String) -> Result<String>,
{
    pub fn new() -> Self {
        let (trigger, tripwire) = Tripwire::new();
        Self {
            //callback: None,
            trigger: Arc::new(Mutex::new(Some(Box::new(trigger)))),
            tripwire: tripwire,
        }
    }

    pub fn on_event(&mut self, callback: StreamCallback) {
        //self.callback = Some(Box::new(callback));
    }

    pub async fn stream(&self) -> Result<String> {
        let mut stream = reqwest::Client::new()
            .get("http://localhost:7331/events")
            .send()
            .await?
            .bytes_stream()
            .eventsource()
            .take_until_if(self.tripwire.clone());

        log::info!("stream created inside");
        while let Some(event) = stream.next().await {
            match event {
                //Ok(event) => match self.callback {
                //    Some(ref c) => {
                //        log::info!("applied callback to event: {}", c(event.data)?)
                //    }
                //    None => log::info!("received event[type={}]: {}", event.event, event.data),
                //},
                Ok(event) => log::info!("received event[type={}]: {}", event.event, event.data),
                Err(e) => log::warn!("stream error occurred: {}", e),
            }
        }
        log::info!("stream end inside");

        Ok("lorem ipsum dolor".into())
    }

    pub async fn cancel(&self) {
        log::info!("stream.cancel: 1");
        let mut locked_trigger = self.trigger.lock().await;
        let owned_trigger = std::mem::replace(locked_trigger.deref_mut(), None);
        log::info!("stream.cancel: 2");
        match owned_trigger {
            Some(trigger) => trigger.cancel(),
            None => log::warn!("trigger not found"),
        }
    }
}

#[cfg(test)]
static_assertions::assert_impl_all!(OpenAiStream: Sync, Send);

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::time::Duration;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[wasm_bindgen_test]
    pub async fn do_test() -> Result<()> {
        wasm_bindgen_test_configure!(run_in_browser);
        console_log::init_with_level(log::Level::Debug)?;
        log::info!("test started");

        let mut stream = Arc::new(OpenAiStream::new());
        //stream.on_event(Box::new(|data| {
        //    log::info!("on_event received data: {}", data);
        //    Ok(data)
        //}));
        //let (trigger, tripwire) = Tripwire::new();

        log::info!("stream created");
        let stream_copy = stream.clone();
        let cancel_stream = async move {
            let duration_secs = 3;
            log::info!("cancelling stream after {duration_secs}s");
            let _ =
                wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &resolve,
                            duration_secs * 1000,
                        )
                        .unwrap();
                }))
                .await;
            stream_copy.cancel().await;
            log::info!("cancelled stream after {duration_secs}s");
        };

        let (_, final_output) = futures::join!(cancel_stream, stream.stream());
        log::info!("stream end with final={:#?}", final_output?);
        anyhow::bail!("test not implemented")
    }
}
