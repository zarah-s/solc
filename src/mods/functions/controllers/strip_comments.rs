use crate::mods::types::types::LineDescriptions;

pub fn strip_comments(lines_: &Vec<LineDescriptions>, _stripped_comments: &mut String) {
    /* STRIP COMMENTS AND WHITE SPACES FROM LINE DESCRIPTORS */
    let stripped_comments: Vec<&LineDescriptions> = lines_
        .iter()
        .filter(|pred| !pred.text.trim().starts_with("//") && !pred.text.trim().is_empty())
        .collect();

    /* STRIPED INLINE COMMENTS */
    let mut stripped_inline_comments: Vec<LineDescriptions> = Vec::new();

    /* STRIP INLINE COMMENTS */
    for stripped_comment in stripped_comments.iter() {
        let comment_index = stripped_comment.text.find("//");
        let doc_str_index = stripped_comment.text.find("/*");
        if let Some(index_value) = comment_index {
            stripped_inline_comments.push(LineDescriptions {
                text: stripped_comment.text[..index_value].trim().to_string(),
                ..**stripped_comment
            })
        } else {
            if let Some(index_value) = doc_str_index {
                if stripped_comment.text.trim() == "/*" {
                    stripped_inline_comments.push(LineDescriptions {
                        text: stripped_comment.text.trim().to_string(),
                        ..**stripped_comment
                    })
                } else {
                    stripped_inline_comments.push(LineDescriptions {
                        text: stripped_comment.text[..index_value].trim().to_string(),
                        ..**stripped_comment
                    });
                }
            } else {
                stripped_inline_comments.push(LineDescriptions {
                    text: stripped_comment.text.trim().to_string(),
                    ..**stripped_comment
                })
            }
        }
    }

    /* JOIN STRIPPED INLINE COMMENTS */
    let joined_stripped_vec: Vec<String> = stripped_inline_comments
        .iter()
        .map(|f| f.to_owned().to_string())
        .collect();

    for sst in &joined_stripped_vec {
        _stripped_comments.push_str(sst.as_str());
    }

    /* STRIP DOC STRINGS */
    while _stripped_comments.contains(&"/*".to_string())
        || _stripped_comments.contains(&"*/".to_string())
    {
        let str_start_index = _stripped_comments.find("/*");
        let str_end_index = _stripped_comments.find("*/");

        if let Some(index_) = str_start_index {
            if let Some(_index) = str_end_index {
                let left: String = _stripped_comments[..index_].to_string();
                let right: String = _stripped_comments[_index + 2..].to_string();

                *_stripped_comments = left + &right;
            }
        }
    }
}
