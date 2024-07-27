
export function fileToBase64(f: File, callback: (f: string) => void): void {
    const reader = new FileReader();
    reader.readAsDataURL(f);
    reader.addEventListener('load', () => callback(reader.result.toString()));
    
}

export function base64ToFile(base64Data: string, filename: string): File {
    // 将Base64字符串转换为二进制字符串
    const binaryString = atob(base64Data.split(',')[1]);
    const mimeString = base64Data.split(',')[0].match(/:([^;]+);/)[1];
 
    // 将二进制字符串转换为ArrayBuffer
    const arrayBuffer = new ArrayBuffer(binaryString.length);
    const intArray = new Uint8Array(arrayBuffer);
    for (let i = 0; i < binaryString.length; i++) {
        intArray[i] = binaryString.charCodeAt(i);
    }
 
    // 使用ArrayBuffer和数据视图创建Blob对象
    const blob = new Blob([intArray], { type: mimeString });
 
    // 使用Blob对象和文件名创建File对象
    return new File([blob], filename, { type: mimeString });
}
 