use anyhow::Result;

use crate::output;
use pretend_core::PretendService;

pub fn run(service: &PretendService, query: &str) -> Result<()> {
  let apps = service.search(query);
  output::print_apps(&apps);
  Ok(())
}
