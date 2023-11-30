use yew::prelude::*;
use web_sys::{ wasm_bindgen::JsCast, HtmlSelectElement, HtmlElement };
// use gloo_console::log;
use crate::render_svg;

#[derive(Clone, Properties, PartialEq, Default)]
pub struct PaginationFucProps {
    pub value: i32,
    pub name: String,
}

#[derive(PartialEq, Properties)]
pub struct PaginatorProps {
    #[prop_or_default]
    pub update_pagination: Callback<PaginationFucProps>,
}

#[derive(Clone, Properties, PartialEq, Default)]
pub struct PaginationDataProps {
    pub current_page: i32,
    pub per_page: i32,
    pub total_items: i32,
    pub total_pages: i32,
}

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

#[function_component(Paginator)]
pub fn paginator(props: &PaginatorProps) -> Html {
    let pagination_data = use_context::<PaginationDataProps>().expect("no ctx found");
    let main_text = format!(
        "Showing {} - {} of {} results",
        pagination_data.per_page.clone() * (pagination_data.current_page.clone() - 1) + 1,
        calculate_per_page(
            pagination_data.per_page.clone(),
            pagination_data.current_page.clone(),
            pagination_data.total_items.clone()
        ),
        pagination_data.total_items.clone()
    );
    let page_numbers = vec![1, 2, 3, 4, 5];

    let max: i32 = pagination_data.total_items.clone();
    let limit_per_page: Vec<i32> = vec![1; max.try_into().unwrap()];

    let handle_previous_page = {
        let has_previous_page = has_previous_page.clone();
        let current_page = pagination_data.current_page.clone();
        let update_pagination = props.update_pagination.clone();
        Callback::from(move |_| {
            if has_previous_page(current_page.clone()) {
                let previous_page = current_page.clone() - 1;
                if previous_page < 1 {
                } else {
                    if previous_page != current_page && current_page > 0 {
                        update_pagination.emit(PaginationFucProps {
                            name: "page".to_string(),
                            value: previous_page,
                        })
                    }
                }
            }
        })
    };

    let handle_next_page = {
        let has_next_page = has_next_page.clone();
        let current_page = pagination_data.current_page.clone();
        let total_pages = pagination_data.total_pages.clone();
        let update_pagination = props.update_pagination.clone();
        Callback::from(move |_| {
            if has_next_page(current_page.clone(), total_pages.clone()) {
                let next_page = current_page.clone() + 1;
                if next_page > total_pages {
                } else {
                    if next_page != current_page && current_page > 0 {
                        update_pagination.emit(PaginationFucProps {
                            name: "page".to_string(),
                            value: next_page,
                        })
                    }
                }
            }
        })
    };

    let update_pagination = props.update_pagination.clone();
    let handle_change_limit = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
        match value.parse::<i32>() {
            Ok(number) => {
                // Successfully parsed the string to a number
                update_pagination.emit(PaginationFucProps {
                    name: "per_page".to_string(),
                    value: number,
                })
            }
            Err(_err) => {}
        }
    });

    html! {
        <div class = "flex flex-col gap-6 md:flex-row justify-start md:justify-between items-center">
            <div class = "flex flex-col gap-6 md:flex-row space-x-4 items-center justify-start md:justify-between">
                <p>{main_text}</p>
                <Pagination
                    page_numbers={page_numbers}
                    current_page={pagination_data.current_page.clone()}
                    total_pages={pagination_data.total_pages.clone()}
                    update_pagination={props.update_pagination.clone()}
                />
            </div>
            <div class = "flex space-x-4 items-center justify-start md:justify-between">
                <p>{"Items per page"}</p>
                <select onchange={handle_change_limit.clone()} class="focus:outline-none p-1 border-grey-shade-6 rounded">
                    {
                        limit_per_page.iter().enumerate().map(|(index, _)| {
                            html!{
                                <option>{index + 1}</option>
                            }
                        }).collect::<Html>()
                        }
                </select>
                <span onclick={handle_previous_page} class={if has_previous_page(pagination_data.current_page.clone()) {"cursor-pointer"} else {"cursor-not-allowed"}}>
                    {html! { render_svg!("ic:round-arrow-left", color="#000000",width="24px")}}
                </span>
                <span  onclick={handle_next_page} class={if has_next_page(pagination_data.current_page.clone(), pagination_data.total_pages.clone()) {"cursor-pointer"} else {"cursor-not-allowed"}}>
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
    pub update_pagination: Callback<PaginationFucProps>,
    pub total_pages: i32,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let active_style = "bg-grey-shade-8 text-grey-shade-0";

    let handle_set_current_page = {
        let update_pagination = props.update_pagination.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            match id.parse::<i32>() {
                                Ok(number) => {
                                    // Successfully parsed the string to a number
                                    update_pagination.emit(PaginationFucProps {
                                        name: "current_page".to_string(),
                                        value: number,
                                    })
                                }
                                Err(_err) => {}
                            }
                        }
                    }
                }
            }
        })
    };

    let page_numbers: Vec<Html> = props.page_numbers
        .iter()
        .map(|&page_num| {
            let number = calculate_page(page_num.clone(), props.current_page.clone());
            html! {
                <>
                    {
                        if calculate_page(page_num.clone(), props.current_page.clone()) > 0 &&
                        calculate_page(page_num.clone(), props.current_page.clone()) <= props.total_pages.clone() {
                            html! {
                                <li data-id={number.to_string()} onclick={handle_set_current_page.clone()} class={format!("py-1 px-2 rounded-sm cursor-pointer {}", if page_num == props.current_page.clone() { active_style } else { "hover:bg-grey-shade-9" })}>
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
