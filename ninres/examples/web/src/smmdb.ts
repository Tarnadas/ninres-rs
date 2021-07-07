import { NinResFile, NinResFileExt } from '../../../pkg/ninres';

import { ninres } from '.';

export async function parseFile(file: File): Promise<NinResFileExt> {
  const buffer = await readFile(file);
  return parseData(new Uint8Array(buffer));
}

export async function parseData(buffer: Uint8Array): Promise<NinResFileExt> {
  console.log('Processing file...');
  const ninresFile = ninres.NinResFileExt.fromBytes(buffer);
  console.log('Processing complete');
  switch (ninresFile.getFileType()) {
    case NinResFile.Sarc:
      ninresFile.getSarc()?.getSfatNodes();
  }
  return ninresFile;
}

async function readFile(file: File): Promise<ArrayBuffer> {
  return new Promise(resolve => {
    const reader = new FileReader();
    reader.addEventListener('loadend', () => {
      resolve(reader.result as ArrayBuffer);
    });
    reader.readAsArrayBuffer(file);
  });
}
