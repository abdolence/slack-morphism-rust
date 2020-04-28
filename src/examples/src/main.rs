use slack_morphism_models as slack;
//use slack::blocks::block_text::*;
use slack::blocks::kit::*;

fn main() {
    let sb: SlackSectionBlock = SlackSectionBlock::new().with_block_id("test".into());
    let sb_ser = serde_json::to_string_pretty(&sb).unwrap();
    let sb_des: SlackSectionBlock = serde_json::from_str(&sb_ser).unwrap();
    println!("{} {:?}", sb_ser, sb_des);
}
