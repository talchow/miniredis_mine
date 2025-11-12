use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use anyhow::Result;

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let cmd = use_state(String::new);
    let resp = use_state(String::new);

    let onsubmit = {
        let cmd = cmd.clone();
        let resp = resp.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let cmd = (*cmd).clone();
            let resp = resp.clone();
            spawn_local(async move {
                match send_to_redis(&cmd).await {
                    Ok(r) => resp.set(r),
                    Err(e) => resp.set(format!("Error: {e}")),
                }
            });
        })
    };

    let oninput = {
        let cmd = cmd.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            cmd.set(input.value());
        })
    };

    html! {
        <div style="font-family:monospace;margin:20px;">
            <h1>{ "MiniRedis Web Terminal" }</h1>
            <form {onsubmit}>
                <input
                    type="text"
                    placeholder="input command here, such as: get key or set key value"
                    value={(*cmd).clone()}
                    {oninput}
                    style="width:400px;"
                />
                <button type="submit">{ "Submit" }</button>
            </form>
            <pre>{ &*resp }</pre>
        </div>
    }
}

/// 与后端的 HTTP 代理交互（浏览器无法直接建立原始 TCP 到 Redis 的连接）
async fn send_to_redis(cmd: &str) -> Result<String> {
    // 这里假设你在后端暴露了一个 HTTP endpoint（例如 /api/redis），由后端将请求转发到 Redis
    // 并返回 Redis 的响应文本。浏览器环境不能直接建立 TCP 连接到 Redis。
    let resp = Request::post("http://127.0.0.1:8080/api/redis")
        .header("Content-Type", "text/plain")
        .body(cmd)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    let text = resp
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;

    Ok(text)
}