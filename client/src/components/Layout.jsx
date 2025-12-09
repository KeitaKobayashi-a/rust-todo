import Box from "@mui/material/Box";

export default function BoxBasic() {
  return (
    <Box
      sx={{
        position: "relative",
        display: "flex",
        overflow: "hidden",
        height: "100vh",
        width: "100%",
        bgcolor: 'red'
      }}
    >
      This Box renders as an HTML section element.
    </Box>
  );
}
