
import React, { useState } from "react";
import { getNovel } from "./novelService";

function Novels({ novels }) {
  const [selectedNovel, setSelectedNovel] = useState(null);

  const handleClick = async (novelId) => {
    const novel = await getNovel(novelId);
    setSelectedNovel(novel);
  };

  if (selectedNovel) {
    return (
      <div>
        <h2>{selectedNovel.title}</h2>
        <ul>
          {selectedNovel.chapters.map((chapter, index) => (
            <li key={index}>
              <h3>{chapter.title}</h3>
              <p>{chapter.content}</p>
            </li>
          ))}
        </ul>
      </div>
    );
  }

  return (
    <div>
      <h2>Available Novels</h2>
      <ul>
        {novels.map((novelId) => (
          <li key={novelId}>
            <button onClick={() => handleClick(novelId)}>{novelId}</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Novels;
