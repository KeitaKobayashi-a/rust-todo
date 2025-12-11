import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import Checkbox from "@mui/material/Checkbox";
import IconButton from "@mui/material/IconButton";
import DeleteIcon from '@mui/icons-material/Delete';

export default function TodoItem({todo, removeTodo, toggleTodo, setTodos}) {
    const labelId = `checkbox-list-label-${todo.id}`;
    const deleteTodo = async () => {
        const data = await fetch(`/api/todos/${todo.id}`, {method: 'DELETE'});
        const json = await data.json();
        setTodos(json);

    }
    return (
        <ListItem
            secondaryAction={
                <IconButton edge="end" aria-label="comments" onClick={() => {
                    deleteTodo();
                    removeTodo(todo.id)
                }}>
                    <DeleteIcon color={'error'}/>
                </IconButton>
            }
            disablePadding
        >
            <ListItemButton role={undefined} onClick={() => toggleTodo(todo.id)}>
                <ListItemIcon>
                    <Checkbox
                        edge="start"
                        checked={todo.completed}
                        tabIndex={-1}
                        disableRipple
                        inputProps={{"aria-labelledby": labelId}}
                    />
                </ListItemIcon>
                <ListItemText id={labelId} primary={todo.title} primaryTypographyProps={{
                    sx: {
                        fontSize: 32,
                        fontWeight: 600,
                    },
                }} sx={todo.completed ? {color: '#00a6ed', textDecoration: "line-through"} : {color: 'white'}}/>
            </ListItemButton>
        </ListItem>

    )
}