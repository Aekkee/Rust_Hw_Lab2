use assert_cmd::Command; 

#[test]
fn areacircle() { 
    let mut cmd = Command::cargo_bin("areacircle").unwrap(); 
    cmd.assert().success().stdout("Cirlcle area: 0\n");
}

#[test]
fn temp() { 
    let mut cmd = Command::cargo_bin("tempcal").unwrap(); 
    cmd.arg("CtoF");
    cmd.arg("10");
    cmd.assert().success().stdout("C to F: 50\n");
}

#[test]
fn listplayer() { 
    let mut cmd = Command::cargo_bin("list_players").unwrap(); 
    cmd.assert().success().stdout("Player 1: N/A\nPlayer 2: N/A\n");
}
