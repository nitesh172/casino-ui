use crate::{
    apis::game::{
        create_game,
        delete_game,
        fetch_games,
        Game,
        GameResponse,
        ProviderResponse,
        fetch_providers,
        CategoryResponse,
        fetch_categories,
        create_category,
        delete_category,
    },
    components::organisms::{
        export_button::{ download_csv_file, ExportButton },
        paginator::{ PaginationDataProps, PaginationFucProps, Paginator },
    },
    render_svg,
    stores::auth_store::AuthStore,
    utils::format_dates::format_date_to_readable,
    ToastProps,
};
use gloo_console::log;
use std::ops::Deref;
use web_sys::{ wasm_bindgen::JsCast, HtmlElement, HtmlInputElement, HtmlSelectElement };
use yew::{ platform::spawn_local, prelude::* };
use yewdux::prelude::use_store;

#[function_component(Games)]
pub fn games() -> Html {
    let is_open = use_state(|| false);
    let is_delete_open = use_state(|| false);
    let is_categories_open = use_state(|| false);
    let is_move_open = use_state(|| false);
    let search_text = use_state(|| String::default());
    let initial = use_state(|| true);
    let games = use_state(|| Vec::<GameResponse>::default());
    let pagination = use_state(|| PaginationDataProps {
        per_page: 10,
        total_items: 0,
        total_pages: 0,
        current_page: 1,
    });
    let game_id = use_state(|| String::default());

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let modal_handle = {
        let is_open = is_open.clone();
        let game_id = game_id.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            game_id.set("".to_string());
        })
    };

    let categories_modal_handle = {
        let is_categories_open = is_categories_open.clone();
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_categories_open.set(!*is_categories_open);
            is_open.set(!*is_open);
        })
    };

    let on_ok_response = {
        let is_open = is_open.clone();
        let game_id = game_id.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
            game_id.set("".to_string());
        })
    };

    let move_modal_handle = {
        let is_move_open = is_move_open.clone();
        let is_categories_open = is_categories_open.clone();
        Callback::from(move |_| {
            is_move_open.set(!*is_move_open);
            is_categories_open.set(!*is_categories_open);
        })
    };

    let move_on_ok = {
        let is_move_open = is_move_open.clone();
        let is_categories_open = is_categories_open.clone();
        Callback::from(move |_| {
            is_move_open.set(!*is_move_open);
            is_categories_open.set(!*is_categories_open);
        })
    };

    let categories_on_ok = {
        let is_categories_open = is_categories_open.clone();
        Callback::from(move |_| {
            is_categories_open.set(!*is_categories_open);
        })
    };

    let delete_modal_handle = {
        let is_delete_open = is_delete_open.clone();
        let game_id = game_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            game_id.set("".to_string());
        })
    };

    let on_ok_delete_handle = {
        let is_delete_open = is_delete_open.clone();
        let game_id = game_id.clone();
        Callback::from(move |_| {
            is_delete_open.set(!*is_delete_open);
            game_id.set("".to_string());
        })
    };

    let delete_callback = {
        let is_delete_open = is_delete_open.clone();
        let game_id = game_id.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            is_delete_open.set(!*is_delete_open);
                            game_id.set(id.clone());
                        }
                    }
                }
            }
        })
    };

    let update_pagination = {
        let pagination = pagination.clone();
        let cloned_initial = initial.clone();
        Callback::from(move |option: PaginationFucProps| {
            cloned_initial.set(true);
            let mut data = pagination.deref().clone();
            let name = option.name;
            let value = option.value;

            match name.as_str() {
                "current_page" => {
                    data.current_page = value;
                }
                "per_page" => {
                    data.per_page = value;
                    data.current_page = 1;
                }
                "total_items" => {
                    data.total_items = value;
                }
                "total_pages" => {
                    data.total_pages = value;
                }
                _ => (),
            }
            pagination.set(data);
        })
    };

    let cloned_search_text = search_text.clone();
    let cloned_initial = initial.clone();
    let search_text_changed = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        cloned_initial.set(true);
        cloned_search_text.set(value);
    });

    let t_head = vec!["Game name", "Category", "Provider", "Actions"];

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let export = {
        let games = games.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_: MouseEvent| {
            let mut csv_data = "Game name|Category|Provider".to_string();
            let add_toast = add_toast.clone();

            for game in games.iter() {
                let data: String = format!(
                    "\n{}|{}|{}",
                    game.clone().name,
                    game.clone().category.name,
                    game.clone().provider.name
                );
                csv_data = csv_data + data.as_str();
            }

            download_csv_file(csv_data.as_str(), add_toast)
        })
    };

    let cloned_games = games.clone();
    let cloned_pagination = pagination.clone();
    let cloned_initial = initial.clone();
    let cloned_search_text = search_text.clone();
    let token = token.clone();
    let fetch_handle_games = move |()| {
        let games = cloned_games.clone();
        let pagination = cloned_pagination.clone();
        let cloned_initial = cloned_initial.clone();
        let search_text = cloned_search_text.clone();
        let token = token.clone();
        spawn_local(async move {
            let response = fetch_games(
                token,
                pagination.deref().clone(),
                search_text.to_string()
            ).await;

            match response {
                Ok(response) => {
                    games.set(response.result);
                    cloned_initial.set(false);
                    pagination.set(PaginationDataProps {
                        current_page: response.page,
                        per_page: response.per_page,
                        total_items: response.total,
                        total_pages: response.total_pages,
                    });
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    {
        let onfetch = fetch_handle_games.clone();
        let pagination = pagination.clone();
        let search_text = search_text.clone();
        use_effect_with((pagination.clone(), search_text.clone()), move |_| {
            if *initial {
                onfetch(());
            }

            || {}
        });
    }

    html! {
        <>
        <div class="bg-grey-shade-13 px-8 py-4">
            <div class="container mx-auto md:w-auto space-y-4">
                <div class="flex md:justify-between md:items-center flex-col gap-6 md:flex-row">
                    <div class="flex flex-row justify-between md:justify-normal gap-2 items-center w-full md:w-auto">
                        <h1>{"Games"}</h1>
                        <div class="flex items-center rounded border justify-start bg-white border-grey-shade-11 px-2 w-40">
                            <span>{html! { render_svg!("mynaui:search",  color="#000000", width="18px")}} </span>
                            <input
                                id="search"
                                autocomplete="off"
                                name="search"
                                placeholder={"search"}
                                onchange={search_text_changed.clone()}
                                class="px-2.5 py-2 h-7 bg-white placeholder:text-grey-shade-6 text-14  leading-20 font-300 font-sans outline-none pr-2 pl-2 w-full "
                            />
                        </div>
                    </div>
                    <div class="flex flex-row items-center w-full md:w-auto gap-2">
                        <ExportButton export={export.clone()} />
                        <button
                            onclick={modal_handle.clone()}
                            class="bg-primary flex flex-1 md:flex-none items-center rounded justify-center lg:justify-normal p-2 text-grey-shade-14 text-12 font-400"
                        >
                            <span class="pr-2">
                            {html! { render_svg!("lets-icons:add-round", color="#ffff",width="14px")}}
                            </span>
                            {"Create"}
                        </button>
                    </div>
                </div>
                <ContextProvider<PaginationDataProps> context={(*pagination).clone()}>
                    <Paginator update_pagination={update_pagination.clone()} />
                </ContextProvider<PaginationDataProps>>
            </div>
        </div>
        <div class="relative px-8">
        <div class="absolute hidden xl:block -z-10 top-0 left-0 h-[45px] w-full bg-grey-shade-13"></div>
        <div class="container mx-auto">
            <table class="w-full table-auto hidden xl:inline-table">
                <thead class="bg-grey-shade-13">
                    <tr>
                        {
                            t_head.clone().into_iter().map(|name| {
                                html!{<th key={name} class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{name}</th>}
                            }).collect::<Html>()
                        }
                    </tr>
                </thead>
                <tbody class="overflow-y-auto">
                    {
                        games.clone().iter().map(|game| {
                            html!{
                                <tr>
                                    <td class="py-3  text-left text-14 font-medium text-grey-shade-1 tracking-wider">{game.clone().name}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider">
                                        <span class="bg-grey-shade-11 rounded-full py-1 px-2">
                                            {game.clone().category.name}
                                        </span>
                                    </td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider font-uppercase pr-5">{game.clone().provider.name}</td>
                                    <td class="py-3 text-left text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                        <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                        <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                            <li onclick={delete_callback.clone()} data-id={game.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">
                                                { "Remove" }
                                            </li>
                                        </ul>
                                    </td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
            <div class="grid md:grid-cols-2 lg:grid-cols-3 xl:hidden gap-4 mt-4">
                {
                    games.clone().iter().map(|game| {
                        html!{
                            <div class="rounded-xl border p-4 flex flex-col justify-between gap-4">
                                <div class="flex flex-row justify-between">
                                    <div class="flex flex-row gap-3 justify-between">
                                        <div class="flex flex-col">
                                            <div class="text-12 text-grey-shade-5">{"Game name"}</div>
                                            <div class="text-14 font-400">{game.clone().name}</div>
                                        </div>
                                        <div class="flex flex-col">
                                            <div class="text-12 text-grey-shade-5">{"Provider"}</div>
                                            <div class="text-14 font-400">{game.clone().provider.name}</div>
                                        </div>
                                    </div>
                                    <div class="text-12 font-medium text-grey-shade-1 relative group cursor-pointer">
                                        <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                        <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-10">
                                            <li onclick={delete_callback.clone()} data-id={game.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Remove"}</li>
                                        </ul>
                                    </div>
                                </div>
                                <div class="rounded-full py-1 px-2 text-12 flex-row gap-1 w-fit bg-grey-shade-11">
                                    {game.clone().category.name}
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
        </div>
        {
            if *is_open {
                html! {
                    <Modal
                        modal_handle={modal_handle.clone()}
                        on_ok_response={on_ok_response.clone()}
                        fetch_handle_games={fetch_handle_games.clone()}
                        game_id={game_id.to_string()}
                        categories_modal_handle={categories_modal_handle.clone()}
                    />
                }
            } else {
                html! ("")
            }
        }
        {
            if *is_delete_open {
                html! {
                    <DeleteModal
                        delete_modal_handle={delete_modal_handle.clone()}
                        on_ok_delete_response={on_ok_delete_handle.clone()}
                        fetch_handle_games={fetch_handle_games.clone()}
                        game_id={game_id.to_string()}
                    />
                }
            } else {
                html! {}
            }
        }
        {
            if *is_categories_open {
                html! {
                    <CategoriesModal
                        modal_handle={categories_modal_handle.clone()}
                        on_ok={categories_on_ok.clone()}
                        move_modal_handle={move_modal_handle.clone()}
                    />
                }
            } else {
                html! {}
            }
        }
        {
            if *is_move_open {
                html! {
                    <MoveModal
                        modal_handle={move_modal_handle.clone()}
                        on_ok={move_on_ok.clone()}
                        fetch_handle_games={fetch_handle_games.clone()}
                    />
                }
            } else {
                html! {}
            }
        }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ModalProps {
    modal_handle: Callback<MouseEvent>,
    categories_modal_handle: Callback<MouseEvent>,
    on_ok_response: Callback<()>,
    fetch_handle_games: Callback<()>,
    game_id: String,
}

#[function_component(Modal)]
fn edit_modal(props: &ModalProps) -> Html {
    let game = use_state(|| Game::default());
    let providers = use_state(|| Vec::<ProviderResponse>::default());
    let categories = use_state(|| Vec::<CategoryResponse>::default());

    let game_list = vec![
        "Blackjack",
        "Roulette",
        "Poker",
        "Baccarat",
        "Craps",
        "Pai Gow Poker",
        "Sci Bo"
    ];

    let cloned_game = game.clone();
    let handle_change = Callback::from(move |event: Event| {
        let mut data = cloned_game.deref().clone();
        let name = event.target().unwrap().unchecked_into::<HtmlSelectElement>().name();
        let value = event.target().unwrap().unchecked_into::<HtmlSelectElement>().value();

        match name.as_str() {
            "provider_id" => {
                data.provider_id = value;
            }
            "category_id" => {
                data.category_id = value;
            }
            _ => (),
        }
        cloned_game.set(data);
    });

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let handle_click = {
        let data = game.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            let mut game = data.deref().clone();
                            let mut vec = data.name.clone();
                            if !vec.contains(&id.clone()) {
                                vec.push(id.clone());
                            } else {
                                let index = vec
                                    .iter()
                                    .position(|x| *x == id.clone())
                                    .unwrap();
                                vec.remove(index);
                            }
                            game.name = vec;
                            data.set(game)
                        }
                    }
                }
            }
        })
    };

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let cloned_providers = providers.clone();
    let cloned_token = token.clone();
    let fetch_handle_providers = move |()| {
        let providers = cloned_providers.clone();
        let token = cloned_token.clone();
        spawn_local(async move {
            let response = fetch_providers(token).await;

            match response {
                Ok(response) => {
                    providers.set(response.result);
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let cloned_categories = categories.clone();
    let cloned_token = token.clone();
    let fetch_handle_categories = move |()| {
        let categories = cloned_categories.clone();
        let token = cloned_token.clone();
        spawn_local(async move {
            let response = fetch_categories(token).await;

            match response {
                Ok(response) => {
                    categories.set(response.result);
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let save_handler = {
        let st = (*game).clone();
        let on_ok = props.on_ok_response.clone();
        let on_handle_games = props.fetch_handle_games.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_event: MouseEvent| {
            let game: Game = st.clone();
            let on_ok = on_ok.clone();
            let on_handle_games = on_handle_games.clone();
            let token = token.clone();
            let add_toast = add_toast.clone();
            spawn_local(async move {
                let response = create_game(token, game).await;

                match response {
                    Ok(_response) => {
                        on_ok.emit(());
                        on_handle_games.emit(());
                        add_toast.emit("Games added successfully.".to_string())
                    }
                    Err(e) => log!("Error: ", e.to_string()),
                }
            });
        })
    };

    let onfetch = fetch_handle_providers.clone();
    let onfetch_category = fetch_handle_categories.clone();
    use_effect_with((), move |_| {
        onfetch(());
        onfetch_category(());
        || {}
    });

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7 w-full md:w-auto">
                <div class="flex items-center justify-between">
                    <p>{"Add game"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>

                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="game"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Platform name"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select name="provider_id" onchange={handle_change.clone()} class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option selected={game.clone().provider_id == ""} >{"Select"}</option>
                            {
                                providers.iter().map(|provider| {
                                    html!{
                                        <option value={provider.clone().id}>{provider.clone().name}</option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                </div>
                {
                    if !game.clone().provider_id.is_empty() {
                        html!{
                            <div class="md:w-[600px]">
                                <ul class="flex flex-wrap w-full gap-2 items-baseline justify-start">
                                    {
                                        game_list.iter().map(|g_item| {
                                            html!{
                                                <li onclick={handle_click.clone()} data-id={g_item.to_string()} class={format!("text-12 px-3 py-2 rounded-lg cursor-pointer {}", if game.clone().name.contains(&g_item.to_string()) {"bg-grey-shade-2 text-grey-shade-14"} else {"bg-grey-shade-13 text-grey-shade-1"})}>
                                                    {g_item}
                                                </li>
                                            }
                                        }).collect::<Html>()
                                    }
                                </ul>
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }
                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="category"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Category"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select name="category_id" onchange={handle_change.clone()}  id="category" class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option selected={game.clone().category_id == ""} >{"Select"}</option>
                            {
                                categories.iter().map(|category| {
                                    html!{
                                        <option value={category.clone().id}>{category.clone().name}</option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                    <div onclick={props.categories_modal_handle.clone()} class="text-sm mt-2 cursor-pointer">{"+ Create new category"}</div>
                </div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={save_handler.clone()}  class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Add"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Cancel"}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CategoriesModalProps {
    modal_handle: Callback<MouseEvent>,
    on_ok: Callback<()>,
    move_modal_handle: Callback<MouseEvent>,
}

#[function_component(CategoriesModal)]
fn categories_modal(props: &CategoriesModalProps) -> Html {
    let categories = use_state(|| Vec::<CategoryResponse>::default());
    let category = use_state(|| String::default());

    let t_heads = vec!["Category", "Created date", "Actions"];

    let cloned_category = category.clone();
    let handle_change = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();

        cloned_category.set(value);
    });

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let cloned_categories = categories.clone();
    let cloned_token = token.clone();
    let fetch_handle_categories = move |()| {
        let categories = cloned_categories.clone();
        let token = cloned_token.clone();
        spawn_local(async move {
            let response = fetch_categories(token).await;

            match response {
                Ok(response) => {
                    categories.set(response.result);
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let add_handler = {
        let st = (*category).clone();
        let on_handle_categories = fetch_handle_categories.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_event: MouseEvent| {
            let category = st.clone();
            let on_handle_categories = on_handle_categories.clone();
            let token = token.clone();
            let add_toast = add_toast.clone();
            spawn_local(async move {
                let response = create_category(token, category).await;

                match response {
                    Ok(_response) => {
                        on_handle_categories(());
                        add_toast.emit("Category added successfully.".to_string());
                    }
                    Err(e) => log!("Error: ", e.to_string()),
                }
            });
        })
    };

    let delete_callback = {
        let on_handle_categories = fetch_handle_categories.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(target) = event.target() {
                if let Some(li) = target.dyn_ref::<HtmlElement>() {
                    if let Some(id) = li.get_attribute("data-id") {
                        if !id.is_empty() {
                            let on_handle_categories = on_handle_categories.clone();
                            let token = token.clone();
                            let add_toast = add_toast.clone();
                            spawn_local(async move {
                                let response = delete_category(token, id).await;

                                match response {
                                    Ok(_response) => {
                                        on_handle_categories(());
                                        add_toast.emit(
                                            "Category removed successfully.".to_string()
                                        );
                                    }
                                    Err(e) => log!("Error: ", e.to_string()),
                                }
                            });
                        }
                    }
                }
            }
        })
    };

    let onfetch = fetch_handle_categories.clone();
    use_effect_with((), move |_| {
        onfetch(());

        || {}
    });

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7 w-full md:w-auto">
                <div class="flex items-center justify-between">
                    <p>{"Categories"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>

                <div class="flex flex-col space-y-1.5 w-full md:w-[400px]">
                    <label
                        for="name"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Category name"}
                    </label>
                    <div class="flex items-center rounded border border-grey-shade-11 justify-start px-2" >
                        <input
                            id="name"
                            name="name"
                            placeholder="Name"
                            value={(*category).clone()}
                            onchange={handle_change.clone()}
                            class="px-3.5 py-3placeholder:text-grey-shade-6 text-14 leading-20
                            bg-white
                            h-10
                            w-full  
                            md:w-72
                            border-grey-shade-11
                            font-300 font-sans outline-none
                            pr-2 pl-2"
                        />
                    </div>
                </div>
                
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={add_handler.clone()} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Add"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Cancel"}
                    </button>
                </div>
                <div class="max-h-36 overflow-scroll overflow-x-hidden">
                <table class="w-full table-auto relative">
                    <thead class="sticky top-0 bg-white z-10">
                        <tr class="">
                            {
                                t_heads.into_iter().map(|name| {
                                    html!{<th key={name} class="py-3 text-left text-14 font-medium text-grey-shade-5 tracking-wider">{name}</th>}
                                }).collect::<Html>()
                            }
                        </tr>
                    </thead>
                    <tbody class="overflow-y">
                        {
                            categories.clone().iter().map(|category| {
                                html!{
                                    <tr>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1">{category.clone().name}</td>
                                        <td class="py-3 text-left text-14 font-medium text-grey-shade-1">{format_date_to_readable(category.clone().created_at, "%d %h %Y")}</td>
                                        <td class="py-3 text-center text-14 font-medium text-grey-shade-1 tracking-wider relative group cursor-pointer">
                                            <span > {html! { render_svg!    ("icon-park:more-one", color="#000000",width="24px")}}</span>
                                            <ul class="hidden absolute -left-10 -mt-1 space-y-2 group-hover:block  py-2 rounded-lg shadow-md shadow-grey-shade-0/10 group-hover:bg-grey-shade-14 z-30">
                                                <li onclick={delete_callback.clone()} data-id={category.clone().id} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Remove"}</li>
                                                <li onclick={props.move_modal_handle.clone()} class="px-4 py-2 text-grey-shade-0 hover:text-grey-shade-2  hover:bg-grey-shade-12 ">{"Move"}</li>
                                            </ul>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                    </tbody>
                </table>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct MoveModalProps {
    modal_handle: Callback<MouseEvent>,
    on_ok: Callback<()>,
    fetch_handle_games: Callback<()>,
}

#[function_component(MoveModal)]
fn move_modal(props: &MoveModalProps) -> Html {
    let categories = use_state(|| Vec::<CategoryResponse>::default());

    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let _toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let cloned_categories = categories.clone();
    let cloned_token = token.clone();
    let fetch_handle_categories = move |()| {
        let categories = cloned_categories.clone();
        let token = cloned_token.clone();
        spawn_local(async move {
            let response = fetch_categories(token).await;

            match response {
                Ok(response) => {
                    categories.set(response.result);
                }
                Err(e) => log!("Error: {}", e.to_string()),
            }
        });
    };

    let onfetch = fetch_handle_categories.clone();
    use_effect_with((), move |_| {
        onfetch(());

        || {}
    });

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7 w-full md:w-auto">
                <div class="flex items-center justify-between">
                    <p>{"Move"}</p>
                    <span class="cursor-pointer" onclick={props.modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div class="font-400 text-16">{"This category contains games move the games from the current category to another category"}</div>
                <div class="flex flex-col space-y-1.5 md:w-[600px]">
                    <label
                        for="name"
                        class="text-11 leading-25 font-sans font-400 text-grey-shade-0"
                    >
                            {"Category name"}
                    </label>
                    <div class="relative inline-flex w-full ">
                        <select name="name" class="appearance-none border border-gray-300 rounded px-3 py-2 focus:outline-none w-full ">
                            <option>{"Select"}</option>
                            {
                                categories.iter().map(|category| {
                                    html!{
                                        <option value={category.clone().id}>{category.clone().name}</option>
                                    }
                                }).collect::<Html>()
                            }
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                            <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 11.586l3.293-3.293a1 1 0 011.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </div>
                    </div>
                </div>
                
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Move"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.modal_handle.clone()}>
                        {"Cancel"}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DeleteModalProps {
    #[prop_or_default]
    on_ok_delete_response: Callback<()>,
    fetch_handle_games: Callback<()>,
    game_id: String,
    delete_modal_handle: Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
fn delete_modal(props: &DeleteModalProps) -> Html {
    let (auth_store, _) = use_store::<AuthStore>();

    let token = auth_store.token.clone();

    let toasts_data = use_context::<ToastProps>().expect("no ctx found");

    let delete_game_handler = {
        let on_ok = props.on_ok_delete_response.clone();
        let on_handle_games = props.fetch_handle_games.clone();
        let game_id = props.game_id.clone();
        let token = token.clone();
        let add_toast = toasts_data.add_toast.clone();
        Callback::from(move |_event: MouseEvent| {
            let on_ok = on_ok.clone();
            let on_handle_games = on_handle_games.clone();
            let game_id = game_id.clone();
            let token = token.clone();
            let add_toast = add_toast.clone();
            spawn_local(async move {
                if !game_id.is_empty() {
                    let response = delete_game(token, game_id).await;

                    match response {
                        Ok(_response) => {
                            on_ok.emit(());
                            on_handle_games.emit(());
                            add_toast.emit("Game removed successfully.".to_string())
                        }
                        Err(e) => log!("Error: ", e.to_string()),
                    }
                }
            });
        })
    };

    html! {
        <div class="z-10 fixed inset-0 bg-grey-shade-0/70 w-screen flex h-screen items-center justify-center p-5">
            <div class=" bg-white p-7 rounded-sm space-y-7">
                <div class="flex items-center justify-between">
                    <p>{"Confirmation required"}</p>
                    <span class="cursor-pointer" onclick={props.delete_modal_handle.clone()}>
                        {html! {render_svg!("mdi:multiply",color="#232323",width="25px")}}
                    </span>
                </div>
                <div>{"Are you sure you want to proceed with this action?"}</div>
                <div class="flex space-x-4  items-center justify-start p-0 rounded-sm">
                    <button type="button" onclick={delete_game_handler} class="bg-primary flex items-center rounded p-2 text-grey-shade-14 text-12 font-400">
                        {"Confirm"}
                    </button>
                    <button class="bg-grey-shade-14 flex items-center rounded p-2 text-primary text-12 font-400" onclick={props.delete_modal_handle.clone()}>
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}
