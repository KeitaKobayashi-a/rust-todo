import * as React from "react";
import List from "@mui/material/List";

import TodoItem from "./TodoItem";
import TodoForm from "./TodoForm";
import {useEffect} from "react";
import {Box} from "@mui/material";
import Typography from "@mui/material/Typography";


export default function TodoList() {
    const [todos, setTodos] = React.useState([]);
    useEffect(
        () => {
            localStorage.setItem('todos', JSON.stringify(todos))
        }, [todos]
    )

    useEffect(() => {
        const getInitialData = async () => {
            const data = await fetch('/api/todos')
            if (!data) return [];
            const json = await data.json()
            setTodos(json)
        }
        getInitialData()
    }, []);

    const removeTodo = id => {
        setTodos(pre => pre.filter(t => t.id !== id))
    };
    const toggleTodo = (id) => {
        setTodos(pre => {
            return pre.map(todo => {
                if (todo.id === id) {
                    return {...todo, completed: !todo.completed}
                } else {
                    return todo
                }
            })
        })
    };
    const addTodo = text => setTodos(pre => {
        return [...pre, {text, id: crypto.randomUUID(), completed: false}]
    });

    return (
        <Box sx={{
            display: 'flex', justifyContent: 'center',
            flexDirection: 'column', alignItems: 'center',
            m: 4
        }}>
            <Typography variant="h1" sx={{color: 'white'}}>
                Todo
            </Typography>

            <List sx={{width: "100%", maxWidth: 360, bgcolor: "background.paper"}}>
                {todos.map(todo => <TodoItem key={todo.id} todo={todo} removeTodo={() => removeTodo(todo.id)}
                                             toggleTodo={toggleTodo}/>
                )}
                <TodoForm addTodo={addTodo}/>
            </List>
        </Box>
    );

}
