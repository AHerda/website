use yew::prelude::*;

use website::{contents::{DataList, DataEntry}, messages::Messages};

pub struct DataListWrapper(DataList, u32);

impl DataListWrapper {
    pub fn new(data_list: DataList) -> Self {
        Self(data_list, 0)
    }
}

impl std::ops::Deref for DataListWrapper {
    type Target = DataList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DataListWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Component for DataListWrapper {
    type Message = Messages;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut data_list = Self::new(DataList::new("Tata Rozpiska".to_string(), vec![]));
        data_list.add_entry(DataEntry::new(0, "Zakupy".to_string(), None, 100.0));
        data_list
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Messages::AddEntry(entry) => {
                self.add_entry(entry);
                self.1 += 1;
            },
            Messages::RemoveEntry(id) => {
                self.remove_entry(id);
                self.1 -= 1;
            },
            Messages::UpdateEntry(entry) => {
                self.update_entry(entry);
            },
        };
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <div>
                <h1>{ &self.name }</h1>
                <ul>
                    { for self.get_data().iter().map(|entry| html! {
                        <li>
                            { entry.id }
                            { " " }
                            { &entry.name }
                            { " " }
                            {
                                entry.date.as_ref().map_or("".to_string(), |date| match date.year {
                                    Some(year) => format!("{:02}/{:02}/{:04}", date.day, date.month, year),
                                    None => format!("{:02}/{:02}", date.day, date.month),
                                })
                            }
                            { " " }
                            { entry.cost }
                        </li>
                    }) }
                </ul>
            </div>
        )
    }
}
