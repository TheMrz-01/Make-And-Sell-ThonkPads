use raylib::prelude::*;
    use raylib::prelude::Rectangle;
    use std::collections::HashMap;


    struct player{
        position: Vector2,
        p_rectangle: Rectangle,
        speed: f32,
        //acceleration: f32,
        //collider: Vector3,
        color: Color,
        keys: HashMap<KeyboardKey, (raylib::prelude::Vector2, raylib::prelude::Vector2)>,
        camera: Camera2D,
    }

    impl player {
        fn new(position: Vector2,speed: f32,color: Color) -> Self{
            let keys = [
                (KeyboardKey::KEY_UP, (Vector2::new(0.0, -speed), Vector2::new(0.0, speed))),
                (KeyboardKey::KEY_DOWN, (Vector2::new(0.0, speed), Vector2::new(0.0, -speed))),
                (KeyboardKey::KEY_LEFT, (Vector2::new(-speed, 0.0), Vector2::new(speed, 0.0))),
                (KeyboardKey::KEY_RIGHT, (Vector2::new(speed, 0.0), Vector2::new(-speed, 0.0))),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();

            let mut camera = Camera2D {
                target: Vector2::new(position.x + 20.0, position.y + 20.0),
                // offset: Vector2::new(player.x, player.y),
                offset: Vector2::new(0.0, 0.0),
                rotation: 0.0,
                zoom: 1.0,
            };

            Self {
                position,
                p_rectangle: Rectangle::new(position.x, position.y, 20.0, 20.0),
                speed,
                color,
                keys,
                camera,
            }
        }

        fn update(&mut self,d: &mut RaylibMode2D<'_,RaylibDrawHandle<'_>>,collThingys: &mut HashMap<&str,Rectangle>){
            let mut new_position = self.position;

            for (_, rect) in collThingys {
                if self.p_rectangle.check_collision_recs(rect) {
                    // Determine the direction of the collision
                    let dx = (self.p_rectangle.x + self.p_rectangle.width / 2.0)
                        - (rect.x + rect.width / 2.0);
                    let dy = (self.p_rectangle.y + self.p_rectangle.height / 2.0)
                        - (rect.y + rect.height / 2.0);

                    let overlap_x = (self.p_rectangle.width + rect.width) / 2.0 - dx.abs();
                    let overlap_y = (self.p_rectangle.height + rect.height) / 2.0 - dy.abs();

                    if overlap_x < overlap_y {
                        if dx > 0.0 {
                            new_position.x += overlap_x;
                        } else {
                            new_position.x -= overlap_x;
                        }
                    } else {
                        if dy > 0.0 {
                            new_position.y += overlap_y;
                        } else {
                            new_position.y -= overlap_y;
                        }
                }
            }
        }
            
            //self.camera.target = Vector2::new(self.position.x + 20.0, self.position.y + 20.0);
            
            for (key, movement) in &self.keys {
                if d.is_key_down(*key) {
                    new_position += movement.0 * d.get_frame_time();
                    //self.camera.offset += movement.1  * d.get_frame_time(); 
                    //self.camera.offset = Vector2::new(self.position.x, self.position.y);
                    //println!("{:?}",self.camera.offset);
                    //self.camera.rotation += movement.1.x; 
                }
            }

            self.position = new_position;
            self.p_rectangle.x = self.position.x;
            self.p_rectangle.y = self.position.y;

            //self.camera.target = Vector2::new(self.position.x + 20.0, self.position.y + 20.0);
            //self.camera.target = Vector2::new(self.position.x + self.p_rectangle.width / 2.0, self.position.y + self.p_rectangle.height / 2.0);

            d.draw_rectangle_rec(self.p_rectangle, self.color);
        }
    }

    fn main() {
        const screen_width: f32 = 640.0;
        const screen_height: f32 = 480.0;

        const redish_orange_shit: Color = Color::new(255, 59, 15,255);
        const dark_gray_shit: Color = Color::new(41, 41, 41,255);

        let mut obstacle_rect = Rectangle::new(300.0, 200.0, 100.0, 100.0);

        let mut collThingys = HashMap::new();
        collThingys.insert("id1", obstacle_rect);

        let mut player1 = player::new(Vector2::new(120.0,120.0), 120.0,redish_orange_shit);
        //----------------------------------------------------------
        let (mut rl, thread) = raylib::init()
            .size(screen_width as i32, screen_height as i32)
            .title("Make And Sell ThonkPads")
            .vsync()
            .build();

        let mut camera = Camera2D {
            //target: Vector2::new(player1.position.x, player1.position.y),
            target: Vector2::new(player1.position.x + player1.p_rectangle.width / 2.0, player1.position.y + player1.p_rectangle.height / 2.0),
            //offset: Vector2::new(player1.position.x, player1.position.y),
            //offset: Vector2::new(0.0, 0.0),
            //offset: Vector2::new(screen_width/2.0,screen_height/2.0),
            offset: Vector2::new(screen_width / 2.0, screen_height / 2.0),
            rotation: 0.0,
            zoom: 1.0,
        };

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            //camera.target = Vector2::new(player1.position.x + player1.p_rectangle.width / 2.0, player1.position.y + player1.p_rectangle.height / 2.0);

            d.clear_background(dark_gray_shit);
            d.draw_fps(0, 0);
            {
                let mut d2 = d.begin_mode2D(camera);
                d2.draw_rectangle_rec(obstacle_rect, Color::WHITE);  
                camera.target = Vector2::new(player1.position.x + player1.p_rectangle.width / 2.0, player1.position.y + player1.p_rectangle.height / 2.0);
                player1.update(&mut d2,&mut collThingys);
                //camera.target = Vector2::new(player1.position.x + player1.p_rectangle.width / 2.0, player1.position.y + player1.p_rectangle.height / 2.0);
            }

            /*d.clear_background(dark_gray_shit);
            d.draw_fps(0, 0);

            d.draw_rectangle_rec(obstacle_rect, Color::WHITE);  
            player1.update(&mut d,&mut collThingys);*/
        }
    }