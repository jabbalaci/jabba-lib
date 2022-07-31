use jabba_lib::jprocess as jproc;

fn main() {
    let commands = vec![
        r#"python -c "print('Hello Py3!')""#,
        "python --version",
        "date",
    ];

    for cmd in commands.iter() {
        let stat = jabba_lib::jprocess::get_exitcode_stdout_stderr(cmd).unwrap();
        println!("{:?}", stat);
    }

    let date = jabba_lib::jprocess::get_exitcode_stdout_stderr("date")
        .unwrap()
        .trimmed_output();
    println!("{:?}", date);

    let cmd = "ls -al";
    jproc::exec_cmd(cmd);
}
