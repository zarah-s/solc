use crate::mods::types::types::LineDescriptions;

pub fn structure_to_line_descriptors(file_contents: &String, lines_: &mut Vec<LineDescriptions>) {
    for (index, content) in file_contents.lines().enumerate() {
        lines_.push(LineDescriptions {
            line: (index as i32) + 1,
            text: content.to_string(),
        })
    }
}
