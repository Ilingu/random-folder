interface Props {
  ChooseFolder: () => void;
}

const Form = ({ ChooseFolder }: Props) => {
  return (
    <form onSubmit={(e) => e.preventDefault()}>
      <button onClick={ChooseFolder}>Choose Folder</button>
    </form>
  );
};

export default Form;
