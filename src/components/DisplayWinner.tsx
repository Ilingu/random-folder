import { useMemo } from "preact/hooks";
import { OpenFolder } from "../utils/fsUtils";

interface Props {
  FolderPath: string;
  ChooseFolder: () => void;
  PickWinner: () => void;
}

const DisplayWinner = ({ FolderPath, ChooseFolder, PickWinner }: Props) => {
  const Name = useMemo(() => FolderPath.split("\\").at(-1), [FolderPath]);

  return (
    <div id="WinnerDir">
      <div className="box" onClick={() => OpenFolder(FolderPath)}>
        <span class="name">[{Name?.slice(0, 50)}]</span>
        <span class="path">{FolderPath}</span>
      </div>

      <section>
        <button onClick={PickWinner}>Pick Again 🔀</button>
        <button onClick={ChooseFolder}>Or Choose Another Folder 📁</button>
      </section>
    </div>
  );
};

export default DisplayWinner;
