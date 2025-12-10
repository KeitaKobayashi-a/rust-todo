import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';

export default function ButtonAppBar({setOpen}) {
    return (
        <Box sx={{flexGrow: 1}}>
            <AppBar position="fixed" sx={{bgcolor: 'black'}}>
                <Toolbar>
                    <IconButton
                        size="large"
                        edge="start"
                        color="info"
                        aria-label="menu"
                        onClick={() => setOpen(true)}
                        sx={{mr: 2}}
                    >
                        <MenuIcon/>
                    </IconButton>
                    <Typography variant="h6" component="div" sx={{flexGrow: 1}}>
                        Rust Todo
                    </Typography>
                    <Button color='info'>Login</Button>
                </Toolbar>
            </AppBar>
        </Box>
    );
}
