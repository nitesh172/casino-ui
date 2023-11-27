use yew::prelude::*;

use crate::{ render_svg, utils::render_svg };
use gloo_console::log;

#[derive(PartialEq, Properties)]
pub struct PaginatorProps {
    pub per_page: i32,
    pub total_pages: i32,
    pub total_items: i32,
    pub current_page: i32,
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

fn calculate_page(page_position: i32, current_page: i32) -> i32 {
    return current_page + (page_position - 1);
}

fn calculate_per_page(per_page: i32, current_page: i32, total_items: i32) -> i32 {
    let new_per_page = per_page.clone() * current_page.clone();
    if new_per_page < total_items.clone() {
        return new_per_page;
    } else {
        return total_items.clone();
    }
}

fn has_previous_page(current_page: i32) -> bool {
    if current_page > 1 {
        return true;
    } else {
        return false;
    }
}

fn has_next_page(current_page: i32, total_pages: i32) -> bool {
    if current_page < total_pages {
        return true;
    } else {
        return false;
    }
}

fn change_page(previous_page: i32, current_page: i32) {
    if previous_page != current_page && current_page > 0 {
        //   setPage(previous_page)
    }
}

fn go_to_previous_page(current_page: i32) {
    let previous_page = current_page.clone() - 1;
    if previous_page < 1 {
    } else {
        change_page(previous_page, current_page)
    }
}

#[function_component(Paginator)]
pub fn paginator(props: &PaginatorProps) -> Html {
    let main_text = format!(
        "Showing {} - {} of {} results",
        props.per_page.clone() * (props.current_page.clone() - 1) + 1,
        calculate_per_page(
            props.per_page.clone(),
            props.current_page.clone(),
            props.total_items.clone()
        ),
        props.total_items.clone()
    );
    let page_numbers = vec![1, 2, 3, 4, 5];

    let handle_previous_page = {
        let has_previous_page = has_previous_page.clone();
        let current_page = props.current_page.clone();
        Callback::from(move |_| {
            if has_previous_page(current_page.clone()) {
                go_to_previous_page(current_page.clone());
            }
        })
    };

    html! {
        <div class = "flex flex-col md:flex-row justify-start md:justify-between items-center">
            <div class = "flex flex-col md:flex-row space-x-4 items-center justify-start md:justify-between">
                <p>{main_text}</p>
                <Pagination
                    page_numbers={page_numbers}
                    current_page={props.current_page.clone()}
                    total_pages={props.total_pages.clone()}
                />
            </div>
            <div class = "flex space-x-4 items-center justify-start md:justify-between">
                <p>{"Items per page"}</p>
                <select class="focus:outline-none p-1 border-grey-shade-6 rounded">
                    <option selected={true}>{"10"}</option>
                    <option>{"20"}</option>
                    <option>{"30"}</option>
                    <option>{"40"}</option>
                    <option>{"50"}</option>
                </select>
                <span onclick={handle_previous_page} class={if has_previous_page(props.current_page.clone()) {"cursor-pointer"} else {"cursor-not-allowed"}}>
                    {html! { render_svg!("ic:round-arrow-left", color="#000000",width="24px")}}
                </span>
                <span class={if has_next_page(props.current_page.clone(), props.total_pages.clone()) {"cursor-pointer"} else {"cursor-not-allowed"}}>
                    {html! { render_svg!("ic:round-arrow-right", color="#000000",width="24px")}}
                </span>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct PaginationProps {
    pub page_numbers: Vec<i32>,
    pub current_page: i32,
    pub total_pages: i32,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let active_style = "bg-grey-shade-8 text-grey-shade-0";

    let page_numbers: Vec<Html> = props.page_numbers
        .iter()
        .map(|&page_num| {
            html! {
                <>
                    {
                        if calculate_page(page_num.clone(), props.current_page.clone()) > 0 &&
                        calculate_page(page_num.clone(), props.current_page.clone()) <= props.total_pages.clone() {
                            html! {
                                <li class={format!("py-1 px-2 rounded-sm cursor-pointer {}", if page_num == props.current_page.clone() { active_style } else { "hover:bg-grey-shade-9" })}>
                                    // { page_num }
                                    {
                                        if calculate_page(page_num.clone(), props.current_page.clone()) <= props.total_pages.clone() 
                                            && calculate_page(page_num.clone(), props.current_page.clone()) > 0 {
                                            calculate_page(page_num.clone(), props.current_page.clone())
                                        } else {
                                            0
                                        }
                                    }
                                </li>
                            }
                        } else {
                            html! {}
                        }
                    }
                </>
            }
        })
        .collect();

    html! {
        <ul class="flex items-center space-x-2">
            { for page_numbers.into_iter() }
        </ul>
    }
}

// {`w-8 h-8 hover:bg-[#481699] hover:text-[#F6F5FF] text-sm rounded ${
//     calculatePage(i) <= totalPages &&
//     calculatePage(i) > 0
//       ? 'cursor-pointer '
//       : 'cursor-default'
//   } ${
//     calculatePage(i) == page
//       ? `bg-[#481699] text-[#F6F5FF]`
//       : `hover:bg-[#481699] hover:text-[#F6F5FF]`
//   }`}
