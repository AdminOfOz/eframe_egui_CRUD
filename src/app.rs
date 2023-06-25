use egui_extras::{Column, TableBuilder, RetainedImage};
use eframe::{egui::{Direction, Layout}};
use egui::emath::Numeric;
use reqwest;
use std::collections::{HashMap,HashSet};
use std::ops::{Add};
use egui::{Align};
use std::thread::park_timeout;
use std::time::{Instant, Duration};
use validator::{Validate};

#[derive(validator::Validate)]
struct SubmitData {
    #[validate(email, contains = "@corp-domain.com", length(min = 20, max = 50))]
    mail: String,
    #[validate(length(min = 1, max = 50))]
    mandatory: String,
    #[validate(url)]
    col2: String,
    #[validate(length(min = 1))]
    col3: String,
    #[validate(length(min = 1))]
    col4: String,
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    req_col: String,
    find: String,
    tint: egui::Color32,
    // this how you opt-out of serialization of a member
    value: f32,
    test_data: Vec<[String; 4]>,
    #[serde(skip)]
    image: RetainedImage,
    col2: String,
    col3: String,
    col4: String,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    selected_resp: String,
    get_resp: Vec<String>,
    usr_email: String,
    found_record: usize,
    pub status: String
}



#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
impl Default for TemplateApp {
    fn default() -> Self {
        let  test_data = Vec::with_capacity(2);
        Self {
            // Example stuff:
            req_col: "value 1".to_owned(),
            find: "".to_owned(),
            col2: "value 2".to_owned(),
            col3: "value 3".to_owned(),
            value: 2.7,
            dropped_files: vec!(),
            picked_path: Option::None,
            col4: "value 4".to_owned(),
            test_data,
            image: RetainedImage::from_image_bytes(
                "../assets/icon-cloud-data.png",
                include_bytes!("../assets/icon-cloud-data.png"),

            ).unwrap(),
            tint: egui::Color32::from_rgb(52, 175, 194),
            selected_resp: "".to_string(),
            get_resp: vec!["Other".to_string(); 1],
            usr_email: "".to_string(),
            found_record: 0.to_owned(),
            status: "Good".to_owned()
        }
    }
}



impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            req_col: req_field,
            find: string,
            col2: col2_val,
            col3: col3_val,
            value,
            dropped_files,
            picked_path,
            test_data,
            col4: opt_bool,
            image,
            tint,
            selected_resp: client,
            get_resp: available_clients,
            usr_email,
            found_record,
            status
        } = self;

        if test_data.len() == 0 {
            test_data.push([
                "buffer_protection".to_string(),
                col2_val.to_string(),
                col3_val.to_string(),
                opt_bool.to_string()
            ])
        }


        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!

        // ---------------------------- //
        //                              //
        //                              //
        //      Begin Top Menu Bar      //
        //                              //
        //                              //
        // ---------------------------- //
        egui::TopBottomPanel::top("menu_bar_for_batch").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Request", |ui| {
                    if ui.button("Reset").clicked() {
                        test_data.clear();
                        test_data.push([
                            "buffer_protection".to_string(),
                            "".to_string(),
                            "".to_string(),
                            "".to_string()
                        ]);
                        self.status = "Everything is good.".to_string();
                        ui.reset_style();
                        dropped_files.clear();
                        *picked_path = Option::None;

                        *col3_val = "not_set".to_string();
                        *req_field = "mandatory value".to_string();
                        *col2_val = "not_set".to_string();
                        *opt_bool = "null".to_string();

                        *ui.ctx().memory() = Default::default();
                        TemplateApp::default();
                    }
                    if ui.button("Remove Duplicates").clicked() {
                        remove_dupes(test_data)
                    }
                    if ui.button("Submit").clicked() {
                        println!("Working on Request");
                        if test_data.len() <= 1 {
                            self.status = "Please enter one or more terms.".to_string();
                        } else {
                            self.status = "Data Processing.\nDo NOT Close.".to_string();
                            //post_req(test_data, self.client.to_string(), self.usr_email.to_string());
                            self.status = "Data sent.".to_string();
                        }
                    }
                });
            });
        });

        // ---------------------------- //
        //                              //
        //                              //
        //     Begin Left Side Panel    //
        //                              //
        //                              //
        // ---------------------------- //
        egui::SidePanel::left("side_panel_batch").show(ctx, |ui| {
            ui.heading("Configure Request");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Required Value: ");
                if ui.text_edit_singleline(req_field).enabled() && ui.input().key_pressed(egui::Key::Enter) {
                    if req_field != "Required" && req_field != "" {
                        add_new_keyword(test_data, req_field.to_string(), col2_val.to_string(), col3_val.to_string(), opt_bool.to_string());
                    }
                };
            });
            ui.vertical(|ui| {
                ui.collapsing("Optional Values", |ui| {
                    ui.horizontal(|ui| {
                        ui.radio_value(opt_bool, "true".to_string(), "true");
                        ui.radio_value(opt_bool, "false".to_string(), "false");
                        ui.radio_value(opt_bool, "null".to_string(), "Not Set");
                    });
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.label("Col 2: ")
                            .on_hover_text("Helpful hint. \n(ex: example here)");
                        ui.text_edit_singleline(col3_val);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Col 3: ")
                            .on_hover_text("Helpful hint here. \n(ex: example data.)");
                        ui.text_edit_singleline(col2_val);
                    })
                });
            });

            if ui.button("Manually Add").clicked() {
                add_new_keyword(test_data, req_field.to_string(), col2_val.to_string(), col3_val.to_string(), opt_bool.to_string());
            }


            ui.add_space(30.0);
            ui.horizontal(|ui| {
                ui.label("Drag-and-drop CSV onto the window");
            });

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("Comma Seperated Spreadsheet", &["csv"]).pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                if picked_path[picked_path.len() - 3..].to_string() != "csv" {
                    ui.label("Please choose a csv");
                } else {
                    ui.add_space(10.0);
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                    ui.add_space(5.0);
                    if ui.button("Confirm & add").clicked() {
                        parse_file(picked_path.clone(), test_data, col3_val.to_string(), col2_val.to_string(), opt_bool.to_string());
                    }
                    //let path = PathBuf::new();
                    //self.picked_path = Some(path.display().to_string());
                }
            };

            ui.add_space(30.0);
            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Recently dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.bytes {
                            use std::fmt::Write as _;
                            write!(info, " ({} bytes)", bytes.len()).ok();
                        }
                        ui.label(info.clone());
                        if !info.to_string().ends_with("csv") {
                            ui.label("ONLY CSV FILES ARE SUPPORTED.");
                        }
                        if info.to_string().ends_with("csv") {
                            if ui.button("Add to Pending Request").clicked() {
                                parse_file(info.clone(), test_data, col3_val.to_string(), col2_val.to_string(), opt_bool.to_string());
                            }
                        }
                    }
                });
            }

            preview_files_being_dropped(ctx);

            // Collect dropped files:
            if !ctx.input().raw.dropped_files.is_empty() {
                self.dropped_files = ctx.input().raw.dropped_files.clone();
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
        // ---------------------------- //
        //                              //
        //                              //
        //    Begin Right Side Panel    //
        //                              //
        //                              //
        // ---------------------------- //
        egui::SidePanel::right("right_side_panel_batch").max_width(350.0).min_width(225.0).show(ctx, |ui| {
            //self.image.show(ui);
            ui.add(egui::Image::new(self.image.texture_id(ctx), self.image.size_vec2() * 0.2)
                       .tint(self.tint),
            );
            ui.heading("API Helper");
            let terms_len = test_data.len() - 1;
            let query_cost = (terms_len.to_f64() * 0.005_f64).floor();
            ui.add_space(50.0);
            ui.label("Query: ".to_owned() + &terms_len.to_string() + " terms");
            ui.label("Estimated Cost: $".to_owned() + &query_cost.to_string());
            ui.add_space(25.0);
            //ui.add.button.fill(Color32::from_rgb(52, 175, 194));
            ui.horizontal(|ui| {
                ui.label("API Get Resp: ");
                egui::ComboBox::from_label("")
                    .selected_text(self.selected_resp.to_string())
                    .show_ui(ui, |ui| {
                        for available_client in &self.get_resp {
                            ui.selectable_value(&mut self.selected_resp, available_client.to_string(), available_client.to_string());
                        }
                        //ui.selectable_value(&mut self.client, "Misc".to_string(), "Wheelhouse DMG");
                    });
                if ui.small_button("↻").clicked() {
                    let _ = &self.get_resp.clear();
                    match get_resp() {
                        Ok(updated_resp) => self.get_resp = updated_resp,
                        Err(error) => panic!("Problem opening the file: {:?}", error),
                    };
                }
            });
            ui.horizontal(|ui| {
                ui.label("Email: ")
                    .on_hover_text("Enter your valid corporate email.");
                ui.text_edit_singleline(&mut self.usr_email);
            });

            // Only allow people to submit requests using their corporate email
            // This is useful for identifying users submitting requests.
            if ui.button("Submit").clicked() {
                if self.usr_email.chars().count() < 17 {
                    //this only appears for a fraction of a second. should be pulled into method similar todropped files
                    self.status = "Please enter your corporate email to continue (eg.  name@corp.com)".to_string();
                } else {
                    self.status = "Removing Dupes, validating data and processing...\n Do NOT Close the Program".to_string();
                    post_req(test_data, self.selected_resp.to_string(), self.usr_email.to_string());
                    #[tokio::main]
                    async fn post_req(test_data: &mut Vec<[String; 4]>, selected_resp: String, email: String) -> Result<(), Box<dyn std::error::Error>> {
                        remove_dupes(test_data);
                        let terms_len = test_data.len() - 1;
                        let set_email = email.to_string();
                        if terms_len != 0 {
                            match set_addtl_params(selected_resp.to_string(), terms_len, email).await {
                                Ok(()) => println!("set addtl params"),
                                Err(r#box) => println!("error")
                            };

                            //Testing api endpoint below
                            let post_url = "https://sdfsfsdfsd.free.beeceptor.com/todos";
                            let mut itr = 0;
                            let mut batch_ctr = 0;
                            for item in test_data {
                                itr = itr + 1;
                                batch_ctr = batch_ctr + 1;
                                let valid_item = validate_data(item, set_email.clone());
                                println!("itr: {itr:?}");
                                println!("batch_ctr: {batch_ctr:?}");

                                if terms_len >= 400 && itr >= 410 && batch_ctr >= 410 {
                                    let timeout = Duration::from_secs(1800);
                                    let beginning_park = Instant::now();

                                    let mut timeout_remaining = timeout;
                                    loop {
                                        park_timeout(timeout_remaining);
                                        let elapsed = beginning_park.elapsed();
                                        if elapsed >= timeout {
                                            batch_ctr = 0;
                                            break;
                                        }
                                        println!("Cool your jets! Continuing after 30 minutes. {elapsed:?}");
                                        //TemplateApp { status: update_txt };
                                        //TimeSelf.status = "Cool your jets! Continuing after 600 seconds. {elapsed:?}".to_string();
                                        timeout_remaining = timeout - elapsed;
                                    }
                                }
                                if valid_item[0] == "buffer_protection".to_string() || valid_item[0].to_lowercase().to_string() == "not_set".to_string() {
                                    continue;
                                }
                                let mut map = HashMap::new();
                                //validate_data(item);
                                //let mut headers = reqwest::header::HeaderMap::new();
                                //headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

                                map.insert("mandatory_data".to_string(), valid_item[0].to_string());
                                map.insert("auth_token".to_string(), "place_holder".to_string());
                                map.insert("hidden_data1".to_string(), "place_holder2".to_string());
                                map.insert("callback".to_string(), "place_holder3".to_string());
                                map.insert("hidden_data2".to_string(), "hidden_val".to_string());
                                if valid_item[1].to_lowercase().to_string() != "not_set" {
                                    map.insert("col2".to_string(), valid_item[1].to_string());
                                }
                                if valid_item[2].to_lowercase().to_string() != "not_set" {
                                    map.insert("col3".to_string(), valid_item[2].to_string());
                                }
                                if valid_item[3].to_lowercase().to_string() != "not_set" {
                                    map.insert("true/false".to_string(), valid_item[3].to_string());
                                }
                                map.insert("response_format".to_string(), "json".to_owned());

                                //TemplateApp.status = "Currently sending word " + itr;
                                let client = reqwest::Client::new();
                                let res = client.post(post_url.to_string()).json(&map).send().await?;
                                println!("Req formed: ");
                                println!("{:#?}", client.post(post_url).json(&map));
                                //res.send();
                                println!("req sent {:#?}", res.status());
                                //println!("{:#?}", resp);
                            }
                        }
                        Ok(())
                    }

                    self.status = "Data sent".to_string()
                }
            }
            ui.add_space(50.0);
            ui.horizontal(|ui| {
                ui.label("System Status:");
                ui.label(&self.status);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's



            ui.horizontal_top(|ui| {
                ui.heading("Terms to Submit:");
                ui.add_space(100.0);
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button("🔎").clicked() {
                        //let search_string = Some(String::from(self.find.to_string()));
                        let iter = &mut self.test_data.iter();
                        let results = iter.position(|s| &s[0] == &self.find.clone().to_string());
                        if !results.is_none() {
                            println!("found");
                            self.found_record = results.unwrap()
                            //let scroll = true;
                            //let scroll_pos = found_record;
                            //self.scroll_to_row = Some((found_record, Align::TOP));
                        } else {
                            println!("no results found");
                        };
                    }
                    ui.text_edit_singleline(&mut self.find);
                })
            });


            let table_len = &self.test_data.len();

            if &self.find != "" {
                TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(200.0).at_least(200.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                    .resizable(true)
                    .scroll_to_row(self.found_record, Option::from(Align::TOP))

                    .header(10.0, |mut header| {
                        for text in ["ID", "Remove", "Mandatory", "Col1", "Col2", "True/False"] {
                            header.col(|ui| {
                                ui.label(text);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(45.0, *table_len - 1, |row_idx, mut row| {
                            row.col(|ui| {
                                ui.label(row_idx.clone().add(1).to_string());
                            });
                            row.col(|ui| {
                                if ui.button("x").clicked() {
                                    //borrow produced unused value setting to _ to ignore
                                    let _ = &self.test_data.remove(row_idx);
                                };
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][0]);
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][1]);
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][2]);
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][3]);
                            });
                        });
                        //if scroll {
                        //    self.scroll_to_row(scroll_pos, Align::TOP);
                        // }
                    });
            } else {
                TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(200.0).at_least(200.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .column(Column::initial(10.0).at_least(10.0))
                    .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                    .resizable(true)
                    .header(10.0, |mut header| {
                        for text in ["ID", "Remove", "Mandatory", "Col2", "Col3", "True/False"] {
                            header.col(|ui| {
                                ui.label(text);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(45.0, *table_len - 1, |row_idx, mut row| {
                            row.col(|ui| {
                                ui.label(row_idx.clone().add(1).to_string());
                            });
                            row.col(|ui| {
                                if ui.button("x").clicked() {
                                    //borrow produced unused value setting to _ to ignore
                                    let _ = &self.test_data.remove(row_idx);
                                };
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][0]);
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][1]);
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][2]);
                            });
                            row.col(|ui| {
                                ui.label(&self.test_data[row_idx][3]);
                            });
                        });
                        //if scroll {
                        //    self.scroll_to_row(scroll_pos, Align::TOP);
                        // }
                    });
            }
        });


        fn preview_files_being_dropped(ctx: &egui::Context) {
            use egui::*;
            use std::fmt::Write as _;

            if !ctx.input().raw.hovered_files.is_empty() {
                let mut text = "Dropping files:\n".to_owned();

                for file in &ctx.input().raw.hovered_files {
                    if file.mime != "CSV" {
                        text += "Only .csv files are supported.\n";
                    } else {
                        if let Some(path) = &file.path {
                            write!(text, "\n{}", path.display()).ok();
                        } else if !file.mime.is_empty() {
                            write!(text, "\n{}", file.mime).ok();
                        } else {
                            text += "\n???";
                        }
                    }
                }

                let painter =
                    ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

                let screen_rect = ctx.input().screen_rect();
                painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
                painter.text(
                    screen_rect.center(),
                    Align2::CENTER_CENTER,
                    text,
                    TextStyle::Heading.resolve(&ctx.style()),
                    Color32::WHITE,
                );
            }
        }



        //This is used to parse CSV files
        fn parse_file(file_path: String, test_data: &mut Vec<[String; 4]>, locale: String, geo: String, mobile: String) {
            use std::error::Error;
            use std::fs::File;
            use std::process;

            fn run(file_path: String, test_data: &mut Vec<[String; 4]>, locale: String, geo: String, mobile: String) -> Result<(), Box<dyn Error>> {
                use std::path::Path;
                let path = Path::new(&file_path);
                let file = File::open(path)?;
                let mut rdr = csv::Reader::from_reader(file);
                let len = test_data.len();
                for result in rdr.records() {
                    let record = result?;

                    let kw = &record[0];

                    test_data.push([
                        kw.to_string(),
                        locale.to_string(),
                        geo.to_owned(),
                        mobile.to_string(),
                    ]);
                    println!("{:?}", record);
                }
                if test_data[len][0] == "buffer_protection" {
                    test_data.remove(len);
                }
                if test_data[len - 1][0] == "buffer_protection" {
                    test_data.remove(len - 1);
                }
                test_data.push([
                    "buffer_protection".to_string(),
                    locale.to_string(),
                    geo.to_string(),
                    mobile.to_string()
                ]);
                Ok(())
            }

            if let Err(err) = run(file_path, test_data, locale, geo, mobile) {
                println!("{}", err);
                process::exit(1);
            }
        }

        //add data to request
        //necessary to do this way because otherwise the table causes crashes
        fn add_new_keyword(test_data: &mut Vec<[String; 4]>, kw: String, mut geo: String, mut loc: String, mob: String) {
            let len = test_data.len();

            if kw == "" || kw == " " {
                return;
            }

            //optional params
            //errors on valid input
            match geo.to_lowercase().as_str() {
                "" | " " | "  " | "n" | "none" | "null" | "no" | "not" | "not  set" | "not set" | "not_" | "not_s" | "not_se" | "not_set" => geo = "not_set".to_string(),
                &_ => ()
            }
            match loc.to_lowercase().as_str() {
                "" | " " | "  " | "n" | "none" | "null" | "no" | "not" | "not  set" | "not set" | "not_" | "not_s" | "not_se" | "not_set" => loc = "not_set".to_string(),
                &_ => ()
            }

            test_data.push([
                kw.to_string(),
                loc.to_string(),
                geo.to_string(),
                mob.to_string()
            ]);

            //make sure you are deleting the buffer
            if test_data[len][0].to_string() == "buffer_protection" {
                test_data.remove(len);
            }
            if test_data[len - 1][0].to_string() == "buffer_protection" {
                test_data.remove(len - 1);
            }


            test_data.push([
                "buffer_protection".to_string(),
                loc.to_string(),
                geo.to_string(),
                mob.to_string()
            ]);
            //test_data.push(to_add)
        }

        #[tokio::main]
        async fn get_resp() -> Result<Vec<String>, Box<dyn std::error::Error>> {
            // This endpoint return each peice of normalized data on a separete line.
            // It is useful for having end users only select a predefined option which may frequently
            // change. This way you do not have to recompile every minor update to the list of options.


            let get_url = "https://sdfsfsdfsd.free.beeceptor.com/todos";

            let body = reqwest::get(get_url).await?.text().await?;

            let mut updated_clients = vec![];

            let mut client = "".to_string();
            for char in body.chars() {
                if char != '\n' {
                    client += &char.to_string();
                } else {
                    updated_clients.push(client.to_string());
                    client = "".to_string();
                }
            }

            //res.send();
            //println!("{:#?}", resp);
            return Result::Ok(updated_clients)
        }

        async fn set_addtl_params(selected_client: String, num_of_reqs: usize, requested_by: String) -> Result<(), Box<dyn std::error::Error>> {
            //if you need to send meta data to a processing server this would be where you do that.
            let post_url = "https://sdfsfsdfsd.free.beeceptor.com/todos";
            let mut map = HashMap::new();

            map.insert("get_selected", selected_client.to_string());
            map.insert("requested_by", requested_by.to_string());
            map.insert("num_requests", num_of_reqs.to_string());
            map.insert("processed_req", "0".to_string());


            let res = reqwest::Request::new(reqwest::Method::POST, post_url.parse().unwrap());
            let client = reqwest::Client::new();
            let res = client.post(post_url).json(&map).send().await?;

            //println!("{:#?}", req);
            //res.send();

            println!("data sent to addtl_data api {:#?}", res.status());
            //println!("{:#?}", resp);
            Ok(())
        }

        // For submitting final requests
        fn validate_data(item: &mut [String; 4], usr_email: String) -> &mut [String; 4] {

            // let validate_data = |item: &mut [String; 4]|, |usr_email: String| -> &mut [String; 4], usr_email: String {
            //handle blank keywords
            let entry = SubmitData {
                mail: usr_email.clone(),
                mandatory: item[0].to_string(),
                col2: item[1].to_string(),
                col3: item[2].to_string(),
                col4: item[3].to_string()
            };
            match entry.validate() {
                Ok(_) => (),
                Err(e) => println!("{}",e)
            };
            return item;
        }
        
        fn remove_dupes(test_data: &mut Vec<[String; 4]>) {
            let mut set = HashSet::new();

            for data in test_data.clone() {
                let mut current_len = set.len();
                set.insert(data.clone());
                current_len = current_len + 1;
                if current_len != set.len() {
                    println!("Found Dupe");
                    let index = test_data.iter().position(|r| r == &data).unwrap();
                    test_data.remove(index);
                }
            }
        }
    }
}

