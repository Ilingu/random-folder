import { useEffect, useRef } from "preact/hooks";

interface Props {
  ImageUrl: string | undefined;
}

type ElementSize = { w: number; h: number };
const ImageRenderer = ({ ImageUrl }: Props) => {
  const LastDimension = useRef<ElementSize>();
  const ImageRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    if (!ImageRef?.current) return;
    if (!ImageUrl) {
      if (!LastDimension.current) return;
      ImageRef.current.style.width = `${LastDimension.current.w}px`;
      ImageRef.current.style.height = `${LastDimension.current.h}px`;
    } else {
      ImageRef.current.style.width = "30%";
      ImageRef.current.style.height = "unset";
    }

    const { width, height } =
      ImageRef.current.getBoundingClientRect() || ImageRef.current;
    LastDimension.current = { w: width, h: height };
  }, [ImageUrl]);

  return <img id="DirImg" src={ImageUrl || ""} ref={ImageRef} />;
};

export default ImageRenderer;
