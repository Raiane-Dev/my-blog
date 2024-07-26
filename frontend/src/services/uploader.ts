import axios from "axios";

export default class UploadAdapter {
	public url;
    public loader;
    public _file: any;

    constructor(loader: any , url: any) {
		this.url = url;
		this.loader = loader;
		this.loader.file.then((pic: any) => (this._file = pic));

		this.upload();
	}

	// Starts the upload process.
	upload() {
		const fd = new FormData();
		fd.append("image", this._file); 

		return new Promise((resolve, reject) => {
			axios
				.post(this.url, fd, {
					onUploadProgress: (e: any) => {
						console.log(
							Math.round((e.loaded / e.total) * 100) + " %"
						);
					}
				})
				.then(response => {
					resolve(response);
				})
				.catch(error => {
					reject("Failed");
					console.log("Server Error : ", error);
				});
		});
	}
}