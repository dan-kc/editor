# Editor

WIP but feel free to take a look

```text
src/
├── app.rs     -> holds the state and application logic
├── buffer.rs  -> holds the text buffer abstration
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
├── ui.rs      -> renders the widgets / UI
└── ui/
    └── widgets.rs     -> holds the UI widgets
```

## TODO

-- Flatten errors
-- b, B

-- viewport & some kind of text_area abstraction (look at yazi)

-- flesh out git stuff
-- action builder?
