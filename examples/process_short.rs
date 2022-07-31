use jabba_lib::jprocess as jproc;

fn main() {
    let cmd = "ls -al";
    jproc::exec_cmd(cmd);
}
