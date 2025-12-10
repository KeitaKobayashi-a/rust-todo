import ListItem from "@mui/material/ListItem";
import TextField from "@mui/material/TextField";
import { useState } from "react";
import InputAdornment from "@mui/material/InputAdornment";
import { Create } from "@mui/icons-material";
import IconButton from "@mui/material/IconButton";

export default function TodoForm({addTodo}) {
    const [text, setText] = useState("");
    const handleChange = (evt) => setText(evt.target.value);
    const handleSubmit = evt => {
        evt.preventDefault();
        addTodo(text);
        setText('')
    }

    return (
        <ListItem>
            <form onSubmit={handleSubmit}>
                <TextField
                    id="outlined-basic"
                    label="新規追加"
                    variant="outlined"
                    value={text}
                    onChange={handleChange}
                    slotProps={{
                        input: {
                            endAdornment: (
                                <InputAdornment position="end">
                                    <IconButton aria-label="create todo" edge="end" type="submit">
                                        <Create />
                                    </IconButton>
                                </InputAdornment>
                            ),
                        },
                    }}
                />
            </form>
        </ListItem>
    );
}
