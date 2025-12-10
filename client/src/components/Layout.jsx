import Box from "@mui/material/Box";
import ResponsiveDrawer from './ResponsiveDrawer.jsx'

export default function Layout() {
  return (
    <Box
      sx={{
        position: "relative",
        display: "flex",
        overflow: "hidden",
        height: "100vh",
        width: "100%",
      }}
    >
      <ResponsiveDrawer/>
    </Box>
  );
}
