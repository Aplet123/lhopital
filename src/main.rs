mod term;

use term::{Choice, Choices};

type Room = Choices<'static, usize, usize>;

struct State {
    read_note: bool,
    taken_homework: bool,
    taken_spanish: bool,
    taken_trebuchet: bool,
    tried_bandsaw: bool,
    taken_bat: bool,
    taken_basketball: bool,
    recalled_note: bool,
    trebuchet_placed: bool,
    titans_tied: bool,
    defeated: bool,
    victorious: bool,
}

fn main() {
    let mut state = State {
        read_note: false,
        taken_homework: false,
        taken_spanish: false,
        taken_trebuchet: false,
        tried_bandsaw: false,
        taken_bat: false,
        taken_basketball: false,
        recalled_note: false,
        trebuchet_placed: false,
        titans_tied: false,
        defeated: false,
        victorious: false,
    };
    let c1 = Room {
        choices: vec![&Choice{
            val: "Run away.",
            extra: 0,
        }],
        flavor: "Your parents 7 and 9 always told you tales of the valiant Captain Zero fighting against the tyranny of Infinitus. \
        The stories of their violent and destructive battles always caused your imagination to bloom. \
        But, your friends always said that they didn't exist. That they were just some fairy tale made up by a math teacher. \
        They just said, \"8, Captain Zero and Infinitus aren't real, they can't hurt you.\" \
        Which made it all the more surprising when Captain Zero suddenly appeared. \
        Infinitus let out a booming scream as he charged towards Captain Zero. He wasn't far away. \
        Adrenaline rushs through your body as it takes over your mind.",
        extra: 0
    };
    c1.print();
    c1.get_input();
    let c2 = Room {
        choices: vec![&Choice{
            val: "Run away.",
            extra: 0,
        }],
        flavor: "You're not far enough. You can still hear the rumbling of the monumental battle roaring behind you. \
        The blood rapidly pumping through your body carries you further and faster than you've ever gone before.",
        extra: 0
    };
    c2.print();
    c2.get_input();
    let mut time: i32 = 0;
    let c3 = Room {
        choices: vec![
            &Choice{
                val: "Read the note.",
                extra: 0,
            },
            &Choice{
                val: "I don't have time for this.",
                extra: 1
            },
        ],
        flavor: "You see a old, crumpled note on the ground. It has ancient runes scribbled all over it. \
                 Your time is ticking down, should you read the note?",
        extra: 0
    };
    c3.print();
    let r3 = c3.get_input();
    if r3.extra == 0 {
        time += 1;
        print!("You pick up the note as its old paper flakes in your hands. \
                The note reads: \"Only when the two titans are evenly matched can the mortals hope to survive. \
                If one prevails, all mortals must flock to his side. \
                If you use the ancient magic of multiplication, it may be possible to modify their powers.\" \
                The note fades away and you keep going. ");
        state.read_note = true;
    } else {
        print!("Time is too important. You leave the note on the ground and keep running. ");
    }
    println!();
    let rooms = [&mut Room {
        choices: vec![
            &Choice{
                val: "Enter the school.",
                extra: 1
            }, &Choice{
                val: "Walk around the school.",
                extra: 5
            }, &Choice {
                val: "Go back to the fight.",
                extra: 109
            }
        ],
        flavor: "You see a school, now vacant because of the budlightvirus. \
                 It seems like there might be some useful items or knowledge in there.",
        extra: 0
    }, &mut Room {
        choices: vec![
            &Choice{
                val: "Exit.",
                extra: 0
            }, &Choice{
                val: "Enter the math classroom.",
                extra: 2
            }, &Choice{
                val: "Enter the spanish classroom.",
                extra: 3
            }, &Choice{
                val: "Enter the engineering classroom.",
                extra: 4
            }, &Choice{
                val: "Raid the lockers.",
                extra: 104
            }, &Choice{
                val: "Go through the back door.",
                extra: 5
            }
        ],
        flavor: "You enter the hallway, and the pungent smell of disinfectant fills your nose. \
                 All the classrooms are blocked except for a math class, a spanish class, and an engineering class. \
                 There are also some lockers lining the wall, and some of them appear to be unlocked for sanitation. \
                 There's also a back door to the fields outside.",
        extra: 1
    }, &mut Room {
        choices: vec![
            &Choice{
                val: "Exit.",
                extra: 1
            }, &Choice{
                val: "Take the homework sheets.",
                extra: 100
            }
        ],
        flavor: "You enter the math class, and the scribbles on the board would suggest that it's a calculus class. \
                 They seemed to have been learning about something known as L'Hopital's rule, and there are posters of Captain Zero and Infinitus everywhere. \
                 The only valuable items appear to be some homework sheets.",
        extra: 2
    }, &mut Room {
        choices: vec![
            &Choice{
                val: "Exit.",
                extra: 1
            }, &Choice{
                val: "Take the packet.",
                extra: 101
            }
        ],
        flavor: "You enter the spanish class, and there seems to be nothing special. \
                 There's an extra copy of a spanish packet that no one wanted, because why would they?",
        extra: 3
    }, &mut Room {
        choices: vec![
            &Choice{
                val: "Exit.",
                extra: 1
            }, &Choice{
                val: "Take a trebuchet/catapult thingy.",
                extra: 102
            }, &Choice{
                val: "Take a band saw.",
                extra: 103
            }
        ],
        flavor: "You enter the engineering class, and the smell of dried tears permeates through the room. \
                 Apart from the dried tears and empty diet coke bottles, you see lots of trebuchets and band saws. \
                 Or is it a catapult? Frankly I'm not too sure. I'll have to ask reddit later today.",
        extra: 4
    }, &mut Room {
        choices: vec![
            &Choice{
                val: "Go back.",
                extra: 0
            }, &Choice{
                val: "Go through the back door.",
                extra: 1
            }, &Choice{
                val: "Go to the baseball field.",
                extra: 6
            }, &Choice{
                val: "Go to the basketball court.",
                extra: 7
            }
        ],
        flavor: "You walk to the back side of the school, and almost everything is locked. \
                 However, the baseball field and the basketball court have a short fence, so you can climb it. \
                 There's also a back door to the school.",
        extra: 5
    }, &mut Room {
        choices: vec![
            &Choice{
                val: "Go back.",
                extra: 5
            }, &Choice{
                val: "Pick up the bat.",
                extra: 105
            }, &Choice{
                val: "Play some baseball.",
                extra: 106
            }
        ],
        flavor: "There's a baseball bat lying in the middle of the field. \
                 Heck, you might even be able to play some baseball and enjoy yourself.",
        extra: 6
    }, &mut Room {
        choices: vec![
            &Choice{
                val: "Go back.",
                extra: 5
            }, &Choice{
                val: "Take the basketball.",
                extra: 107
            }
        ],
        flavor: "There are McWend-fil-a bags scattered across the court. \
                 There's also a worn out basketball on the court.",
        extra: 7
    }];
    let mut cur_room: &mut Room = rooms[0];
    loop {
        cur_room.print();
        let next: usize = cur_room.get_input().extra;
        match next {
            100 => {
                cur_room = rooms[2];
                cur_room.flavor =
                    "You've taken the homework sheets and now the classroom looks sad.";
                cur_room.choices.remove(1);
                state.taken_homework = true;
                time += 1;
            }
            101 => {
                cur_room = rooms[3];
                cur_room.flavor =
                    "You've taken the packet and now the room is completely worthless.";
                cur_room.choices.remove(1);
                state.taken_spanish = true;
                time += 1;
            }
            102 => {
                cur_room = rooms[4];
                cur_room.flavor =
                    "You've taken a thing that launches other things that has a name that ends with a 't', and now all that remains are band saws and bad nightmares.";
                cur_room.choices.remove(1);
                state.taken_trebuchet = true;
                time += 1;
            }
            103 => {
                cur_room = rooms[4];
                if !state.tried_bandsaw {
                    cur_room.flavor = "Did you really think you could take the band saws? Those things are huge, and you're just a puny little number. \
                                       I won't stop you from trying again, though.";
                    state.tried_bandsaw = true;
                } else {
                    cur_room.flavor = "Wow you are very dedicated to moving this band saw. \
                                       It won't work. It's really heavy. Stop trying. \
                                       Trust me, you won't get some kind of special message after trying 20 times."
                }
                time += 1;
            }
            104 => {
                cur_room = rooms[1];
                cur_room.flavor = "You wasted tons of time running through the school and opening all the lockers. \
                                   And guess what? There were a total of zero items. You know why? \
                                   Because it's high school. Who still uses their locker?";
                time += 3;
            }
            105 => {
                cur_room = rooms[6];
                cur_room.flavor =
                    "You've taken the baseball bat and you're ready to play some baseball.";
                cur_room.choices.remove(1);
                state.taken_bat = true;
                time += 1;
            }
            106 => {
                if !state.taken_bat {
                    cur_room = rooms[6];
                    cur_room.flavor = "You haven't even taken the bat, silly! How are you going to play baseball without a bat?";
                } else {
                    cur_room = rooms[6];
                    cur_room.flavor = "Okay great, you have a bat, but you have no one to play with. \
                                       You're just running around in circles and you're pretty funny looking.";
                    time += 1;
                }
            }
            107 => {
                cur_room = rooms[7];
                state.taken_basketball = true;
                cur_room.flavor = "You've taken the basketball and it feels great in your hands.";
                cur_room.choices.remove(1);
                cur_room.choices.insert(
                    1,
                    &Choice {
                        val: "Shoot some hoops.",
                        extra: 108,
                    },
                );
                time += 1;
            }
            108 => {
                cur_room = rooms[7];
                cur_room.flavor = "You throw the basketball, and it misses the hoop completely and pops as it hits the ground. \
                                   So much for that I guess.";
                cur_room.choices.remove(1);
                state.taken_basketball = false;
                time += 1;
            }
            109 => {
                break;
            }
            _ => {
                cur_room = rooms[next];
                time += 1;
            }
        }
        if time >= 13 {
            break;
        }
    }
    let mut turn: i32 = 0;
    let mut flav: &'static str = "Your time is up. The battle is nearing an end. Suddenly, your body gets pulled towards the explosive battle. \
                                  Infinitus summoned the spell known as \"5x+8\", originally invented by Line-us Pauling. \
                                  Alas, Captain Zero was done playing around, and he summoned the powerful spell of \"40x^2+71x+21\", \
                                  colloquially known by its latin name, \"matha problema contriveda\". \
                                  You can feel yourself bowing to Captain Zero's sheer power, and you must stop it before it's too late.";
    loop {
        let mut opts: Vec<&Choice<usize>> = vec![];
        if state.defeated {
            opts.push(&Choice {
                val: "Accept defeat.",
                extra: 1337,
            });
        } else if state.victorious {
            opts.push(&Choice {
                val: "Go home.",
                extra: 42,
            });
        } else {
            if state.read_note {
                opts.push(&Choice {
                    val: "Recall the note.",
                    extra: 0,
                });
            }
            if state.taken_basketball {
                opts.push(&Choice {
                    val: "Use the basketball.",
                    extra: 1,
                });
            }
            if state.taken_bat {
                opts.push(&Choice {
                    val: "Use the bat.",
                    extra: 2,
                });
            }
            if state.taken_homework {
                opts.push(&Choice {
                    val: "Use the math homework.",
                    extra: 3,
                });
            }
            if state.taken_spanish {
                opts.push(&Choice {
                    val: "Use the spanish packet.",
                    extra: 4,
                });
            }
            if state.taken_trebuchet {
                opts.push(&Choice {
                    val: "Use the trebuchet/catapult thingy.",
                    extra: 5,
                });
            }
            if state.tried_bandsaw {
                opts.push(&Choice {
                    val: "Use the band saw.",
                    extra: 6,
                });
            }
            opts.push(&Choice {
                val: "Give up.",
                extra: 7,
            });
        }
        let disp = Room {
            choices: opts,
            flavor: flav,
            extra: 0,
        };
        disp.print();
        let choice = disp.get_input().extra;
        if state.defeated || state.victorious {
            break;
        }
        match choice {
            0 => {
                state.read_note = false;
                state.recalled_note = true;
                flav = "You think back to the note you read. \
                        Something about invoking the ancient power of multiplication."
            }
            1 => {
                if !state.trebuchet_placed {
                    flav = "You throw the basketball at Captain Zero and it deflates on his ironclad skin. \
                            Captain Zero turns towards you and you quickly submit to his will.";
                    state.defeated = true;
                } else {
                    flav = "You load the trebuchet then launch the basketball at Captain Zero and it deflates on his ironclad skin. \
                            Captain Zero turns towards you and you quickly submit to his will.";
                    state.defeated = true;
                }
            }
            2 => {
                if state.titans_tied {
                    flav = "You lightly tap (since violence is bad) 1 on the head with a baseball bat. \
                            He faints, and you emerge from the rubble, victorious.";
                    state.victorious = true;
                } else if state.trebuchet_placed {
                    flav = "You awkwardly load the trebuchet with the baseball bat, and you launch it. \
                            It turns out that the trebuchet isn't built for baseball bats and it flew backwards and knocked you out.";
                    state.defeated = true;
                } else {
                    flav = "You tried to get close to swing the bat at Captain Zero but he accidentally stepped on you. \
                            That's how inconsequential mere morals are to him.";
                    state.defeated = true;
                }
            }
            3 => {
                if state.trebuchet_placed {
                    if state.recalled_note {
                        flav = "The note...you understand it now! \
                                You quickly find an \"x\" on the homework sheet, and you tear it out. \
                                You launch it at Infinitus and scream everything you remember about multiplication. \
                                The paper starts to glow, and it fuses with Infinitus. \
                                Suddenly, Infinitus' power becomes \"5x^2+8x\" and he is evenly matched. \
                                They strike at each other, and they knock each other out. \
                                The mortals scurry to steal the spot of power and you hide under a rock. \
                                Eventually, 1 emerged as number 1.";
                        state.taken_homework = false;
                        state.titans_tied = true;
                    } else {
                        flav =
                            "You load the homework on the trebuchet and launch it at Infinitus. \
                                It let off a spark somehow, but it stopped short and did nothing. \
                                Captain Zero turns towards you, and you can't escape.";
                        state.defeated = true;
                    }
                } else {
                    flav = "You wave the homework in the homework in the air, screaming for someone to pay attention to you. \
                            Unfortunately for you, Captain Zero paid attention to you. \
                            In all the wrong ways.";
                    state.defeated = true;
                }
            }
            4 => {
                flav = "It's a spanish packet. I don't know what you expect from it. \
                        Are you hoping that there'll be some section about L'Hopital's but written in spanish? \
                        Because that's not what people learn about in spanish class.";
                state.defeated = true;
            }
            5 => {
                flav = "You place down the pew pew thing and angle it towards the two titans.";
                state.trebuchet_placed = true;
                state.taken_trebuchet = false;
            }
            6 => {
                flav = "You grab the band saw you brought with you using your glorious muscles, and you hurl it at the titans. \
                        They both go down, and you emerge victorious, as the great hero that saved the mortals from the titans. \
                        Except, you don't have a band saw because you're too weak to bring it with you. \
                        You forgot about that part didn't you?";
                state.defeated = true;
            }
            7 => {
                flav = "You sit back, relax, and watch as the titans duke it out and Captain Zero emerges victorious. \
                        You have no choice but to join him.";
                state.defeated = true;
            }
            _ => {}
        }
        turn += 1;
        if turn > 4 {
            flav = "You've taken too long and Infinitus has been defeated. \
                    There is nothing getting between Captain Zero and you.";
            state.defeated = true;
        }
    }
    if state.defeated {
        println!("You couldn't live the fight between Captain Zero and Infinitus this time, but maybe you'll have better luck next time.");
    } else {
        println!("Congratulations! You live to see another day!");
    }
}
