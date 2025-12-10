import Box from "@mui/material/Box";
import ButtonAppBar from "./ButtonAppBar.jsx";
import TemporaryDrawer from "./TemporaryDrawer.jsx";

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
        <TemporaryDrawer/>
    </Box>
  );
}
