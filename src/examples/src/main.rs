use slack_morphism_models as slack;
use slack::blocks::block_text::{SlackBlockPlainText, SlackBlockText};
use slack_morphism_models::blocks::block_text::SlackBlockMarkDownText;


fn main() {
    let bpt : SlackBlockPlainText = SlackBlockPlainText::from("hey");
    let btpt : SlackBlockText = SlackBlockPlainText::new(&String::from("hey")).as_block_text();

    let btmd = SlackBlockMarkDownText::from("hey").as_block_text();

    println!("{:?} {:?}", bpt, btpt);
    println!("{:?}", btmd);
}
