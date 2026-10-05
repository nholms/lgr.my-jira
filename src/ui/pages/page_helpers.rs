use ellipse::Ellipse;

// Simple left or center algnment with padding @ width.
pub fn get_column_aligned(text: &str, width: usize, center: bool) -> String {
    const MAX_WIDTH: usize = 30;

    // Gate conditions
    if width == 0 {
        return "".to_owned();
    } else if width <= 3 {
        return format!("{}", ".".repeat(width));
    }

    // Leaving the option to make it smaller
    let width = width.min(MAX_WIDTH); // Clamp to Max

    let mut res = String::new();

    if text.len() > width {
        let ellip = text.truncate_ellipse(width - 3);

        // Slice from the end
        if let Some(slice) = ellip
            .char_indices()
            .nth_back(width - 1)
            .map(|(i, _)| &ellip[i..])
        {
            res = slice.to_string();
        }
    } else {
        // Will result in padding
        res = text.to_string();
    };

    if center {
        return format!("{:^size$}", &res, size = width); // Centered
    } else {
        return format!("{:<size$}", &res, size = width); // Left Aligned
    }
}

pub fn get_column_string(text: &str, width: usize) -> String {
    get_column_aligned(text, width, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
