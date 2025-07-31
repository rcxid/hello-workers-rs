use chrono::Utc;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use rand::distr::Alphanumeric;
use rand::Rng;

/// 生成随机长度的字符串
fn generate_random_string(length: usize) -> String {
    let mut rng = rand::rng();
    std::iter::repeat(())
        .map(|()| char::from(rng.sample(Alphanumeric)))
        .take(length)
        .collect()
}

/// 计算字符串md5sum
fn md5sum(str: &str) -> String {
    let digest = md5::compute(str.as_bytes());
    format!("{:x}", digest)
}

/// 批量计算
fn hash_calc(count: usize) -> String {
    let start = Utc::now();
    for _i in 0..count {
        let s = generate_random_string(16);
        md5sum(s.as_str());
    }
    let end = Utc::now();
    format!("耗时: {}", (end - start).to_string())
}

#[component]
pub fn HashCalc() -> impl IntoView {
    let (msg, set_msg) = signal("还没有测试结果".to_string());
    let (count, set_count) = signal(100_usize);

    let start_calc = move |_| {
        set_msg.update(|x| *x = "测试中".to_string());
        let m = hash_calc(count.get());
        set_msg.update(|x| *x = m);
    };

    view! {
        <div>
            <fieldset class="fieldset">
              <legend class="fieldset-legend">输入字符串hash测试次数</legend>
              <input
                type="number"
                class="input"
                placeholder="请输入"
                value=move || count.get().to_string()
                on:input=move |ev| {
                    let input_value = event_target_value(&ev);
                    if let Ok(parsed) = input_value.parse::<usize>() {
                        set_count.set(parsed);
                    }
                }
              />
            </fieldset>
            <p>{msg}</p>
            <button class="btn btn-sm btn-soft" on:click=start_calc>"开始测试"</button>
        </div>
    }
}
