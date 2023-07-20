use assert_cmd::Command; 

#[test]
fn areacircle() { 
    let mut cmd = Command::cargo_bin("areacircle").unwrap(); 
    cmd.arg("10");
    cmd.assert().success().stdout("Cirlcle area: 314.15997\n");
}

#[test]
fn temp1() { 
    let mut cmd = Command::cargo_bin("tempcal").unwrap(); 
    cmd.arg("CtoF");
    cmd.arg("10");
    cmd.assert().success().stdout("C to F: 50\n");
}

#[test]
fn temp2() { 
    let mut cmd = Command::cargo_bin("tempcal").unwrap(); 
    cmd.arg("FtoC");
    cmd.arg("10");
    cmd.assert().success().stdout("F to C: -12.222223\n");
}

#[test]
fn listplayer1() { 
    let mut cmd = Command::cargo_bin("list_players").unwrap(); 
    cmd.assert().success().stdout("Player 1: N/A\nPlayer 2: N/A\n");
}

#[test]
fn listplayer2() { 
    let mut cmd = Command::cargo_bin("list_players").unwrap(); 
    cmd.arg("Paul").arg("John");
    cmd.assert().success().stdout("Player 1: Paul\nPlayer 2: John\n");
}

#[test]
fn listplayer3() { 
    let mut cmd = Command::cargo_bin("list_players").unwrap(); 
    cmd.arg("Paul");
    cmd.assert().success().stdout("Player 1: Paul\nPlayer 2: N/A\n");
}


