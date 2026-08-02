use crate::cli::LintArgs;
use crate::model::Project;
use crate::output;
use crate::rules;

pub fn run(args: LintArgs) -> i32 {
    let cwd = std::env::current_dir().unwrap_or_default();
    let project = Project::new(cwd);

    if args.fix {
        let _changed = rules::fix::apply_fixes(&project);
    }

    let violations = rules::run_all(&project);
    let out = if args.json {
        output::json::render(&violations)
    } else {
        output::human::render(&violations)
    };
    print!("{}", out);

    if violations.iter().any(|v| v.is_error()) {
        1
    } else {
        0
    }
}
