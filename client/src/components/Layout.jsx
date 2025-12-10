import Box from "@mui/material/Box";
import Header from "./Header"

export default function BoxBasic() {
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
      <Header/>
    </Box>
  );
}
