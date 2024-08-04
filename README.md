# Description
ER-Save-Lib is a library for reading and writing Elden Ring save files, compatible with PC and Playstation Save Wizard exported saves. This library is currently in alpha and is being developed alongside the new release of [ER-Save-Editor](https://github.com/ClayAmore/ER-Save-Editor).

# Usage
## SaveApi
### Example
```rust
use er_save_lib::SaveApi;

fn main() {
    #[cfg(unix)]
    let null_path = "/dev/null";
    #[cfg(windows)]
    let null_path = "NUL";
    let character_index = 0;

    // PC
    let save_api = SaveApi::from_path("./test/ER0000.sl2");

    match save_api {
        Ok(mut save_api) => {
            save_api
                .set_character_name(character_index, "New Name")
                .expect("");

            save_api.write_to_path(null_path).expect("");
        }
        Err(err) => eprintln!("{err}"),
    }

    // Playstation
    let save_api = SaveApi::from_path("./test/ps_save.txt");

    match save_api {
        Ok(save_api) => {
            let character_name = save_api.character_name(character_index);
            println!("{}", character_name);
            save_api
                .write_to_path(null_path)
                .expect("");
        }
        Err(err) => eprintln!("{err}"),
    }
}
```


## Save
### Example
```rust
use er_save_lib::Save;

fn main() {
    #[cfg(unix)]
    let null_path = "/dev/null";
    #[cfg(windows)]
    let null_path = "NUL";

    // PC
    let save = Save::from_path("./test/ER0000.sl2");

    match save {
        Ok(save) => {
            save.write_to_path(null_path).expect("");
        }
        Err(err) => eprintln!("{err}"),
    }

    // Playstation
    let save = Save::from_path("./test/ps_save.txt");

    match save {
        Ok(save) => {
            save.write_to_vec().expect("");
        }
        Err(err) => eprintln!("{err}"),
    }
}
```

## Credits
<div style="display: column;">
<a href="https://github.com/vswarte/"><img width=100 height=100  src="https://github.com/user-attachments/assets/c79f4130-a990-4b50-8131-5fe938b7573f"/></a>
<a href="https://github.com/nordgaren/"><img width=100 height=100  src="https://github.com/ClayAmore/ER-Save-Editor/assets/131625063/710c9ee6-c3df-4665-be6b-d96bce1ebf46"/>
<a href="https://github.com/ClayAmore/"><img width=100 height=100 src="https://avatars.githubusercontent.com/u/131625063?v=4"/>
</div>