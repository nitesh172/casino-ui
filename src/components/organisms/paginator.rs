use yew::prelude::*;

use crate::{render_svg, utils::render_svg};

#[derive(PartialEq, Properties)]
pub struct PaginatorProps {
    // pub per_page: i32,
    // pub total_pages: i32,
    // pub total_items: i32,
    // pub current_page: i32,
}

// fn calculate_page(page: i32, page_position: i32) -> i32 {
//     return page + (page_position - 1);
// }

// fn has_previous_page(page: i32) -> bool {
//     return page > 1;
// }

// fn has_next_page(page: i32, total_pages: i32) -> bool {
//     return page < total_pages;
// }

// fn goto_next_page(page: i32, total_pages: i32) -> i32 {
//     let next_page = page + 1;
//     if next_page > total_pages {
//         return change_page(page + 1);
//     }
// }

// fn goto_previous_page(page: i32) -> i32 {
//     let previous_page = page - 1;
//     if previous_page < 1 {
//         return page;
//     }
//     return change_page(page - 1);
// }

// fn change_page(current_page: i32) -> () {
//     if (currentPage != page && page > 0) {
//         setPage(currentPage);
//         setIsLoading(true);
//     }
// }

#[derive(Clone, PartialEq, Properties)]
pub struct PaginationProps {
    pub page_numbers: Vec<usize>,
    pub current_page: usize,
}

#[function_component(Paginator)]
pub fn paginator(props: &PaginatorProps) -> Html {
    let main_text = format!("Showing {} - {} of {} results", 1, 10, 20);
    let page_numbers = vec![1, 2, 3, 4, 5];

    html! {
        <div class = "flex flex-col md:flex-row justify-start md:justify-between items-center">
            <div class = "flex flex-col md:flex-row space-x-4 items-center justify-start md:justify-between">
                <p>{main_text}</p>
                <Pagination
                    page_numbers={page_numbers}
                    current_page={1}
                />
            </div>
            <div class = "flex space-x-4 items-center justify-start md:justify-between">
                <p>{"Items per page"}</p>
                <select class="focus:outline-none p-1 border-grey-shade-6 rounded">
                    <option selected=true>{"10"}</option>
                    <option>{"20"}</option>
                    <option>{"30"}</option>
                    <option>{"40"}</option>
                    <option>{"50"}</option>
                </select>
                <span>
                    {html! { render_svg!("ic:round-arrow-left", color="#000000",width="24px")}}
                </span>
                <span>
                    {html! { render_svg!("ic:round-arrow-right", color="#000000",width="24px")}}
                </span>
            </div>
        </div>
    }
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let active_style = "bg-grey-shade-8 text-grey-shade-0";

    let page_numbers: Vec<Html> = props
        .page_numbers
        .iter()
        .map(|&page_num| {
            html! {
                <li class={format!("py-1 px-2 rounded-sm cursor-pointer {}", if page_num == props.current_page { active_style } else { "" })}>
                    { page_num }
                </li>
            }
        })
        .collect();

    html! {
        <ul class="flex items-center space-x-2">
            { for page_numbers.into_iter() }
        </ul>
    }
}
