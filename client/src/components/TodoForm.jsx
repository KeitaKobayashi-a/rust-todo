import ListItem from "@mui/material/ListItem";
import TextField from "@mui/material/TextField";
import {useState} from "react";
import InputAdornment from "@mui/material/InputAdornment";
import {Create} from "@mui/icons-material";
import IconButton from "@mui/material/IconButton";

export default function TodoForm({addTodo, setIsSubmit}) {
    const [text, setText] = useState("");
    const handleChange = (evt) => setText(evt.target.value);
    const handleSubmit = async (evt) => {
        evt.preventDefault();
        addTodo(text);
        setText('')
        const data = await fetch('/api/todos', {
            method: 'POST', headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json',
            }, body:
                JSON.stringify({title: text, description: 'not use'})
        })
        setIsSubmit(pre => !pre)

    }

    return (
        <ListItem >
            <form onSubmit={handleSubmit}style={{ width: "80%" }}>
                <TextField
                    id="outlined-basic"
                    label="new Todo"
                    variant="outlined"
                    value={text}
                    onChange={handleChange}
                    fullWidth
                    slotProps={{
                        input: {
                            endAdornment: (
                                <InputAdornment position="end">
                                    <IconButton aria-label="create todo" edge="end" type="submit">
                                        <Create/>
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
