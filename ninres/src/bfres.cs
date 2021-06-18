        public void Read(FileData f)
        {
            data = f;
            f.seek(4); // magic check
            f.skip(4); // Padding
            f.Endian = Endianness.Little;
            verNumD = f.readByte();
            verNumC = f.readByte();
            verNumB = f.readByte();
            verNumA = f.readByte();
            f.Endian = Endianness.Big;

            Console.WriteLine("Version = " + verNumA + "." + verNumB + "." + verNumC + "." + verNumD);

            if (f.readShort() == 0xFEFF) // 0xC
                f.Endian = Endianness.Big;
            else f.Endian = Endianness.Little;
            f.skip(2); // sizHeader // 0xE
            f.skip(4); // FileNameOffsetToString // 0x10
                f.skip(4); // file alignment usuallt 0x00002000 // 0x14
            int RelocationTableOffset = f.readOffset(); // 0x18
            int BfresSize = f.readOffset(); // 0x1C

            /*Note, alignment is for gpu addresses so not important for this*/

            Text = f.readString(f.readOffset() + 2, -1); // 0x20
            // 
            Console.WriteLine("Reading " + ModelName);

            f.skip(4); // Padding // 0x24
            int FMDLOffset = f.readOffset(); // 0x28
            f.skip(4); // Padding // 0x2C
            int FMDLDict = f.readOffset(); // 0x30
            f.skip(4); // Padding
            int FSKAOffset = f.readOffset();
            f.skip(4); // Padding
            int FSKADict = f.readOffset(); // 0x40
            f.skip(4); // Padding
            int FMAAOffset = f.readOffset();
            f.skip(4); // Padding
            int FMAADict = f.readOffset(); // 0x50
            f.skip(4); // Padding
            int FVISOffset = f.readOffset();
            f.skip(4); // Padding
            int FVISDict = f.readOffset(); // 0x60
            f.skip(4); // Padding
            int FSHUOffset = f.readOffset();
            f.skip(4); // Padding
            int FSHUDict = f.readOffset(); // 0x70
            f.skip(4); // Padding
            int FSCNOffset = f.readOffset();
            f.skip(4); // Padding
            int FSCNDict = f.readOffset(); // 0x80
            f.skip(4); // Padding
            int BuffMemPool = f.readOffset();
            f.skip(4); // Padding
            int BuffMemPoolInfo = f.readOffset(); // 0x90
            f.skip(4); // Padding
            int EMBOffset = f.readOffset();
            f.skip(4); // Padding
            EMBDict = f.readOffset(); // 0xA0
            f.skip(12); // Padding
            int StringTableOffset = f.readOffset(); // 0xB0
            f.skip(4); // Padding
            int unk11 = f.readOffset();
            int FMDLCount = f.readShort(); // 0xBC
            int FSKACount = f.readShort();
            int FMAACount = f.readShort(); // 0xC0
            int FVISCount = f.readShort();
            int FSHUCount = f.readShort();
            int FSCNCount = f.readShort();
            int EMBCount = f.readShort(); // 0xC8
            f.skip(12); // Padding
            Console.WriteLine("EMBDict" + BFRES.EMBDict);

            // INDEX GROUPS

            //This is pretty messy atm. Makes sure offsets don't = 0. 

            TreeNode modelGroup = new TreeNode();
            modelGroup.Text = "Models";
            modelGroup.ImageKey = "folder";
            modelGroup.SelectedImageKey = "folder";
            Nodes.Add(modelGroup);
            if (FMDLOffset != 0)
            {
                f.seek(FMDLDict);
                IndexGroup fmdlGroup = new IndexGroup(f);

                for (int i = 0; i < FMDLCount; i++)
                {
                    f.seek(FMDLOffset + (i * 120));
                    modelGroup.Nodes.Add(new FMDL(f));
                }
            }

            TreeNode animGroup = new TreeNode();
            animGroup.Text = "Skeleton Animations";
            animGroup.ImageKey = "folder";
            animGroup.SelectedImageKey = "folder";
            Nodes.Add(animGroup);
            if (FSKAOffset != 0)
            {
                f.seek(FSKADict);
                IndexGroup fskaGroup = new IndexGroup(f);
                for (int i = 0; i < FSKACount; i++)
                {
                    f.seek(FSKAOffset + (i * 96));
                    animGroup.Nodes.Add(new FSKA(f));
                }
            }

            TreeNode MAAGroup = new TreeNode();
            MAAGroup.Text = "Material Animations";
            MAAGroup.ImageKey = "folder";
            MAAGroup.SelectedImageKey = "folder";
            Nodes.Add(MAAGroup);
            if (FMAAOffset != 0)
            {
                f.seek(FMAAOffset);
                IndexGroup FMAAGroup = new IndexGroup(f);

                for (int i = 0; i < FMAACount; i++)
                {
                    f.seek(FMAAOffset + (i * 120));
                    MAAGroup.Nodes.Add(new FMAA(f));
                }

            }
            TreeNode VISGroup = new TreeNode();
            VISGroup.Text = "Visual Animations";
            VISGroup.ImageKey = "folder";
            VISGroup.SelectedImageKey = "folder";
            Nodes.Add(VISGroup);
            if (FVISOffset != 0)
            {
                f.seek(FVISDict);
                IndexGroup FVISGroup = new IndexGroup(f);
                for (int i = 0; i < FVISCount; i++)
                {
                    f.seek(FVISOffset + (i * 104));
                    VISGroup.Nodes.Add(new FVIS(f));
                }
            }
            TreeNode SHUGroup = new TreeNode();
            SHUGroup.Text = "Shape Animations";
            SHUGroup.ImageKey = "folder";
            SHUGroup.SelectedImageKey = "folder";
            Nodes.Add(SHUGroup);
            if (FSHUOffset != 0)
            {
                f.seek(FSHUDict);
                IndexGroup FSHUGroup = new IndexGroup(f);

                for (int i = 0; i < FSHUCount; i++)
                {
                    f.seek(FMAAOffset + (i * 104));
                    SHUGroup.Nodes.Add(new FSHU(f));
                }
            }
            TreeNode SCNGroup = new TreeNode();
            SCNGroup.Text = "Scene Animations";
            SCNGroup.ImageKey = "folder";
            SCNGroup.SelectedImageKey = "folder";
            Nodes.Add(SCNGroup);
            if (FSCNOffset != 0)
            {
                f.seek(FSCNDict);
                IndexGroup FSCNGroup = new IndexGroup(f);

                for (int i = 0; i < FSCNCount; i++)
                {
                    f.seek(FSCNOffset + (i * 120));
                    SCNGroup.Nodes.Add(new FSCN(f));
                }
            }
            TreeNode embGroup = new TreeNode();
            embGroup.Text = "Embedded Files";
            embGroup.ImageKey = "folder";
            embGroup.SelectedImageKey = "folder";
            Nodes.Add(embGroup);
            if (EMBOffset != 0)
            {
                f.seek(EMBDict);
                IndexGroup fembGroup = new IndexGroup(f);
                for (int i = 0; i < EMBCount; i++)
                {

                    f.seek(EMBOffset + (i * 16));
                   embGroup.Nodes.Add(new ExternalFiles(f) { Text = fembGroup.names[i] });
                }
            }
        }