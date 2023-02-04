// use std::collections::HashMap;
// use std::time::Duration;
//
//
// #[derive(Default, Clone, Debug)]
// pub struct EcsDebugTimer{
//     entries: HashMap<String, Entry>,
//     running_timers: HashMap<String, DT>,
// }
//
//
// #[derive(Default, Clone, Debug)]
// struct Entry{
//     total: Duration,
//     entries: u32
// }
// impl EcsDebugTimer{
//     pub fn new() -> Self{
//         Self{
//             entries: Default::default(),
//             running_timers: Default::default()
//         }
//     }
//     fn get_entry(&mut self, name: String) -> &mut Entry{
//         if !self.entries.contains_key(&name){
//             self.entries.insert(name.clone(), Entry{
//                 total: Duration::from_micros(0),
//                 entries: 0
//             });
//         }
//         return self.entries.get_mut(&name).unwrap();
//     }
//     pub fn start_timer(&mut self, name: String){
//         let timer = DT::start(name.clone().as_str());
//         self.running_timers.insert(name, timer);
//     }
//     pub fn stop_timer(&mut self, name: String){
//         let timer = self.running_timers.remove(&name).unwrap();
//         let time = timer.stop();
//         let mut entry = self.get_entry(name);
//
//         entry.total += time;
//         entry.entries += 1;
//
//     }
//     pub fn print_all(&self){
//         println!(" ---- Times: ---- ");
//         let time_budget_ms = 16.0/20.0;
//         println!("Budget: {}ms", time_budget_ms);
//
//         let mut total_duration = Duration::from_secs(0);
//         let mut sorted: Vec<(&String, &Entry)> = self.entries.iter().collect();
//
//         sorted.sort_by(|(name1, entry1), (name2, entry2)| {
//             entry1.total.cmp(&entry2.total).reverse()
//         });
//
//         for (key, value) in sorted{
//             let average = value.total / value.entries;
//             total_duration += average;
//             println!("{}% - {} - {:?}", average.as_nanos() as f32 / (time_budget_ms * 1_000_000.0) * 100.0, key, average);
//         }
//         println!("Total: {}% {:?}/{}ms",
//                  total_duration.as_nanos() as f32 / (time_budget_ms * 1_000_000.0) * 100.0,
//                  total_duration, time_budget_ms);
//     }
// }