use bevy::prelude::*;


// 玩家组件
#[derive(Component,Debug)]
pub struct Player;


// 敌人
#[derive(Component,Debug)]
pub struct Enemy;

// 子弹
#[derive(Component,Debug)]
pub struct Bullt;


#[derive(Component,Debug)]
pub struct GameScore;


#[derive(Component,Debug)]
pub enum GameMenu {
    Start,
    Quit,
}


#[derive(Component,Debug)]
pub struct OnMainMenuScreen;