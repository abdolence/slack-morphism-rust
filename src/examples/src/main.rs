use slack_morphism_models as slack;
use slack::*;
use slack::blocks::kit::*;

fn main() {
    let sb: SlackSectionBlock = SlackSectionBlock::new().with_block_id("test".into());
    let sb_ser = serde_json::to_string_pretty(&sb).unwrap();
    let sb_des: SlackSectionBlock = serde_json::from_str(&sb_ser).unwrap();
    println!("{} {:?}", sb_ser, sb_des);

    let section_block = SlackSectionBlock::new()
        .with_text( md!("hey, {}", 10) )
        .with_fields(
            slack_items! [
                some(md!("hey1")),
                some(pt!("hey2")),
                optionally( sb_ser.is_empty() => md!("hey"))
            ]
        );

    let blocks : Vec<SlackBlock> =
        slack_blocks! {
            blocks [
               block(section_block.clone()),
               optionally( sb_ser.is_empty() => section_block.clone())
            ]
        };

    println!("{:#?}",blocks);

}
