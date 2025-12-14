use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use anyhow::Result;

use crate::types::{GetRequest, SetRequest};

#[function_component(Terminal)]
pub fn terminal() -> Html {
    // get操作和set操作的状态管理
    let get_key =use_state(||String::new());
    let set_key =use_state(||String::new());
    let set_value =use_state(||String::new());

    // 响应状态管理
    let get_resp = use_state(String::new);
    let set_resp = use_state(String::new);

    // get操作提交事件处理
    let on_get_submit = {
        let get_key = get_key.clone();
        let get_resp = get_resp.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let get_key = (*get_key).clone();
            let get_resp = get_resp.clone();
            spawn_local(async move {
                match send_get_request(&get_key).await {
                    Ok(r) => get_resp.set(r),
                    Err(e) => get_resp.set(format!("Error: {e}")),
                }
            });
        })
    };
    
    // set操作提交事件处理
    let on_set_submit = {
        let set_key = set_key.clone();
        let set_value = set_value.clone();
        let set_resp = set_resp.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let set_key = (*set_key).clone();
            let set_value = (*set_value).clone();
            let set_resp = set_resp.clone();
            spawn_local(async move {
                match send_set_request(&set_key, &set_value).await {
                    Ok(r) => set_resp.set(r),
                    Err(e) => set_resp.set(format!("Error: {e}")),
                }
            });
        })
    };

    // 输入框变化事件处理函数,用于更新状态管理的key值
    let on_get_input = {
        let get_key = get_key.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            get_key.set(input.value());
        })
    };

    // set操作key输入框变化事件处理函数,用于更新状态管理的key值
    let on_set_input = {
        let set_key = set_key.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            set_key.set(input.value());
        })
    };
    
    // set操作value输入框变化事件处理函数,用于更新状态管理的value值
    let on_set_value_input = {
        let set_value = set_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            set_value.set(input.value());
        })
    };

   html! { // font-family:monospace 用于等宽字体显示, margin:20px 增加边距
        <div style="font-family:monospace;margin:20px;">
            <h1 style="text-align: center; color: #e74c3c;">{ "MiniRedis Web Terminal" }</h1>  // 标题
            
            // GET操作区域
            <div style="margin-bottom: 20px; padding: 10px; border: 1px solid #2eab3dff; border-radius: 5px;">
                <h3>{ "Query" }</h3>
                <form onsubmit={on_get_submit}> // 提交表单事件绑定
                    <div style="margin-bottom: 10px;">
                        <label for="get-key" style="display: inline-block; width: 80px;">{ "Email:" }</label>
                        <input
                            id="get-key" // 获取输入框的id
                            type="text"
                            placeholder="Enter email to query username"
                            value={(*get_key).clone()} //绑定get_key状态管理
                            oninput={on_get_input} // 绑定输入框变化事件处理函数
                            style="width: 300px;"
                        />
                    </div>
                    <button type="submit">{ "Confirm" }</button> // 提交按钮
                </form>
               
               // 显示GET操作响应结果
                <div style="margin-top: 15px;">
                    <h4 style="color: #3498db; margin-bottom: 5px;">{ "Query Result:"}</h4>
                    <pre style="
                        background-color: #e8f4fd;
                        padding: 10px;
                        border-radius: 5px;
                        min-height: 30px;
                        border-left: 5px solid #3498db;
                        white-space: pre-wrap;
                        word-wrap: break-word;
                        ">{ &*get_resp }
                    </pre>
                </div>
            </div>

            // SET操作区域
            <div style="margin-bottom: 20px; padding: 10px; border: 1px solid #2eab3dff; border-radius: 5px;">
                <h3>{ "Insert" }</h3>
                <form onsubmit={on_set_submit}>  // 提交表单事件绑定
                    // SET操作key输入框
                    <div style="margin-bottom: 10px;">
                        <label for="set-key" style="display: inline-block; width: 80px;">{ "Email:" }</label>
                        <input
                            id="set-key"  // 设置输入框的id
                            type="text"
                            placeholder="Enter email"
                            value={(*set_key).clone()} // 绑定set_key状态管理
                            oninput={on_set_input}  // 绑定输入框变化事件处理函数
                            style="width: 300px;"
                        />
                    </div>
                    // SET操作value输入框
                    <div style="margin-bottom: 10px;">
                        <label for="set-value" style="display: inline-block; width: 80px;">{ "Username:" }</label>
                        <input
                            id="set-value"
                            type="text"
                            placeholder="Enter username"
                            value={(*set_value).clone()}
                            oninput={on_set_value_input} // 绑定输入框变化事件处理函数
                            style="width: 300px;"
                        />
                    </div>
                    <button type="submit">{ "Confirm" }</button> // 提交按钮
                </form>

                // 显示SET操作响应结果
                <div style="margin-top: 15px;">
                    <h4 style="color: #3498db; margin-bottom: 5px;">{ "Insert Result:"}</h4>
                    <pre style="
                        background-color: #e8f4fd;
                        padding: 10px;
                        border-radius: 5px;
                        min-height: 30px;
                        border-left: 5px solid #3498db;
                        white-space: pre-wrap;
                        word-wrap: break-word;
                        ">{ &*set_resp } // 显示SET操作响应结果
                    </pre>
                </div>
            </div>
        </div>
    }
} 

/// 发送GET请求到后端
async fn send_get_request(key: &str) -> Result<String> {
    // 构建GET请求结构体
    let request = GetRequest {
        key: key.to_string(),
    };
    // 构建GET请求URL
    let url = "http://127.0.0.1:8080/api/get";
    log::info!("尝试连接到: {}", url);
    
    // 手动构建JSON字符串，避免序列化问题
    let json_body = serde_json::to_string(&request)
        .map_err(|e| {
            log::error!("JSON序列化失败: {:?}", e);
            anyhow::anyhow!("JSON序列化失败: {:?}", e)
        })?;
    
    log::info!("请求体: {}", json_body);
    
    // 发送POST请求
    let resp = Request::post(url)
        .header("Content-Type", "application/json")
        .body(json_body)
        .map_err(|e| {
            log::error!("构建GET请求失败: {:?}", e); // 记录构建GET请求失败日志
            anyhow::anyhow!("构建请求失败: {:?}", e)  // 返回错误信息
        })?
        .send()
        .await
        .map_err(|e| {
            log::error!("发送GET请求失败: {:?}", e);
            anyhow::anyhow!("连接失败: {:?} - 请确保后端服务正在运行", e)
        })?;

    log::info!("GET请求状态: {}", resp.status());
    
    // 读取响应体为文本
    let text = resp
        .text()
        .await
        .map_err(|e| {
            log::error!("读取响应失败: {:?}", e);
            anyhow::anyhow!("读取响应失败: {:?}", e)
        })?;

    log::info!("GET响应: {}", text);
    Ok(text)
}

/// 发送SET请求到后端
async fn send_set_request(key: &str, value: &str) -> Result<String> {
    let request = SetRequest {
        key: key.to_string(),
        value: value.to_string(),
    };
    
    let url = "http://127.0.0.1:8080/api/set";
    log::info!("尝试连接到: {}", url);
    
    // 手动构建JSON字符串，避免序列化问题
    let json_body = serde_json::to_string(&request)
        .map_err(|e| {
            log::error!("JSON序列化失败: {:?}", e);
            anyhow::anyhow!("JSON序列化失败: {:?}", e)
        })?;
    
    log::info!("请求体: {}", json_body);
    
    let resp = Request::post(url)
        .header("Content-Type", "application/json")
        .body(json_body)
        .map_err(|e| {
            log::error!("构建SET请求失败: {:?}", e);
            anyhow::anyhow!("构建请求失败: {:?}", e)
        })?
        .send()
        .await
        .map_err(|e| {
            log::error!("发送SET请求失败: {:?}", e);
            anyhow::anyhow!("连接失败: {:?} - 请确保后端服务正在运行", e)
        })?;

    log::info!("SET请求状态: {}", resp.status());
    
    let text = resp
        .text()
        .await
        .map_err(|e| {
            log::error!("读取响应失败: {:?}", e);
            anyhow::anyhow!("读取响应失败: {:?}", e)
        })?;

    log::info!("SET响应: {}", text);
    Ok(text)
}