#[macro_export]
macro_rules! render_svg {
    ($($rest:expr),*) => {
        Html::from_html_unchecked(iconify::svg!($($rest),*).into())
    };
}

// macro_rules! render_svg {
//     ($($rest:expr),*) => {
//         Html::from_html_unchecked(iconify::svg!($($rest),*).into())
//     };
// }
