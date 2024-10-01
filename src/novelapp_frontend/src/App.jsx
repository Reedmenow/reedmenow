
import React, { useState, useEffect } from "react";
import { getNovels, getNovel } from "./novelService";
import Novels from "./Novels";

function App() {
  const [novels, setNovels] = useState([]);

  useEffect(() => {
    async function fetchNovels() {
      const novelList = await getNovels();
      setNovels(novelList);
    }
    fetchNovels();
  }, []);

  return (
    <div className="App">
      <h1>Novel Reader</h1>
      <Novels novels={novels} />
    </div>
  );
}

export default App;
