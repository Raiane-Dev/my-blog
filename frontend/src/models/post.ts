type PostInput = {
    id?: number;
    title?: string;
    body?: string;
    image_path: string;
    description: string;
};

const PostPattern: PostInput = {
    title: "",
    body: "",
    image_path: "",
    description: "",
}
export { 
    PostPattern,

    
    type PostInput
};