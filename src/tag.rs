//! Constants for commonly used tags in TIFF files, baseline 
//! or extended.
//! 
//! Check the [Tiff Tag Reference](https://www.awaresystems.be/imaging/tiff/tifftags.html)
//! for more information on each tag.

#![allow(non_upper_case_globals)]

pub const NewSubfileType: u16 = 0x00FE;
pub const SubfileType: u16 = 0x00FF;
pub const ImageWidth: u16 = 0x0100;
pub const ImageLength: u16 = 0x0101;
pub const BitsPerSample: u16 = 0x0102;
pub const Compression: u16 = 0x0103;
pub const PhotometricInterpretation: u16 = 0x0106;
pub const Threshholding: u16 = 0x0107;
pub const CellWidth: u16 = 0x0108;
pub const CellLength: u16 = 0x0109;
pub const FillOrder: u16 = 0x010A;
pub const DocumentName: u16 = 0x010D;
pub const ImageDescription: u16 = 0x010E;
pub const Make: u16 = 0x010F;
pub const Model: u16 = 0x0110;
pub const StripOffsets: u16 = 0x0111;
pub const Orientation: u16 = 0x0112;
pub const SamplesPerPixel: u16 = 0x0115;
pub const RowsPerStrip: u16 = 0x0116;
pub const StripByteCounts: u16 = 0x0117;
pub const MinSampleValue: u16 = 0x0118;
pub const MaxSampleValue: u16 = 0x0119;
pub const XResolution: u16 = 0x011A;
pub const YResolution: u16 = 0x011B;
pub const PlanarConfiguration: u16 = 0x011C;
pub const PageName: u16 = 0x011D;
pub const XPosition: u16 = 0x011E;
pub const YPosition: u16 = 0x011F;
pub const FreeOffsets: u16 = 0x0120;
pub const FreeByteCounts: u16 = 0x0121;
pub const GrayResponseUnit: u16 = 0x0122;
pub const GrayResponseCurve: u16 = 0x0123;
pub const T4Options: u16 = 0x0124;
pub const T6Options: u16 = 0x0125;
pub const ResolutionUnit: u16 = 0x0128;
pub const PageNumber: u16 = 0x0129;
pub const TransferFunction: u16 = 0x012D;
pub const Software: u16 = 0x0131;
pub const DateTime: u16 = 0x0132;
pub const Artist: u16 = 0x013B;
pub const HostComputer: u16 = 0x013C;
pub const Predictor: u16 = 0x013D;
pub const WhitePoint: u16 = 0x013E;
pub const PrimaryChromaticities: u16 = 0x013F;
pub const ColorMap: u16 = 0x0140;
pub const HalftoneHints: u16 = 0x0141;
pub const TileWidth: u16 = 0x0142;
pub const TileLength: u16 = 0x0143;
pub const TileOffsets: u16 = 0x0144;
pub const TileByteCounts: u16 = 0x0145;
pub const BadFaxLines: u16 = 0x0146;
pub const CleanFaxData: u16 = 0x0147;
pub const ConsecutiveBadFaxLines: u16 = 0x0148;
pub const SubIFDs: u16 = 0x014A;
pub const InkSet: u16 = 0x014C;
pub const InkNames: u16 = 0x014D;
pub const NumberOfInks: u16 = 0x014E;
pub const DotRange: u16 = 0x0150;
pub const TargetPrinter: u16 = 0x0151;
pub const ExtraSamples: u16 = 0x0152;
pub const SampleFormat: u16 = 0x0153;
pub const SMinSampleValue: u16 = 0x0154;
pub const SMaxSampleValue: u16 = 0x0155;
pub const TransferRange: u16 = 0x0156;
pub const ClipPath: u16 = 0x0157;
pub const XClipPathUnits: u16 = 0x0158;
pub const YClipPathUnits: u16 = 0x0159;
pub const Indexed: u16 = 0x015A;
pub const JPEGTables: u16 = 0x015B;
pub const OPIProxy: u16 = 0x015F;
pub const GlobalParametersIFD: u16 = 0x0190;
pub const ProfileType: u16 = 0x0191;
pub const FaxProfile: u16 = 0x0192;
pub const CodingMethods: u16 = 0x0193;
pub const VersionYear: u16 = 0x0194;
pub const ModeNumber: u16 = 0x0195;
pub const Decode: u16 = 0x01B1;
pub const DefaultImageColor: u16 = 0x01B2;
pub const JPEGProc: u16 = 0x0200;
pub const JPEGInterchangeFormat: u16 = 0x0201;
pub const JPEGInterchangeFormatLength: u16 = 0x0202;
pub const JPEGRestartInterval: u16 = 0x0203;
pub const JPEGLosslessPredictors: u16 = 0x0205;
pub const JPEGPointTransforms: u16 = 0x0206;
pub const JPEGQTables: u16 = 0x0207;
pub const JPEGDCTables: u16 = 0x0208;
pub const JPEGACTables: u16 = 0x0209;
pub const YCbCrCoefficients: u16 = 0x0211;
pub const YCbCrSubSampling: u16 = 0x0212;
pub const YCbCrPositioning: u16 = 0x0213;
pub const ReferenceBlackWhite: u16 = 0x0214;
pub const StripRowCounts: u16 = 0x022F;
pub const XMP: u16 = 0x02BC;
pub const ImageID: u16 = 0x800D;
pub const Copyright: u16 = 0x8298;
pub const ImageLayer: u16 = 0x87AC;