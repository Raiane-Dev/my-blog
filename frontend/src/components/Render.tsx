const RenderHTML = ({ html }: any) => {
  return <div dangerouslySetInnerHTML={{ __html: html }} />;
};

export { RenderHTML };
