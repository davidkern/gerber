/// Gerber Commands
/// 
/// Each variant is the "long name" listed in §2.8 of the specification.
/// Variants are also identified by [command code constants](crate::command#constants).
pub enum Command {
    /// [G04] A human readable comment, does not affect the image.
    Comment,

    /// [MO] Sets the unit to mm or inch.
    Mode,

    /// [FS] Sets the coordinate format, e.g. the number of decimals.
    FormatSpecification,

    /// [AD] Defines a template-based aperture, assigns a D code to it.
    ApertureDefine,

    /// [AM] Defines a macro aperture template.
    ApertureMacro,

    /// [D] (Dnn for nn≥10) Sets the current aperture to D code nn.
    SetCurrentAperture,

    /// [D01] Outside a region statement [D01] creates a draw or arc
    /// object with the current aperture. Inside it adds a draw/arc
    /// segment to the contour under construction. The current
    /// point is moved to draw/arc end point after the creation of
    /// the draw/arc.
    Plot,

    /// [D02] Moves the current point to the coordinate in the
    /// command. It does not create an object.
    Move,

    /// [D03] Creates a flash object with the current aperture. The
    /// current point is moved to the flash point.
    Flash,

    /// [G01] Sets linear/circular mode to linear.
    SetLinear,

    /// [G02] Sets linear/circular mode to clockwise circular.
    SetCWCircular,

    /// [G03] Sets linear/circular mode to counterclockwise circular.
    SetCCWCircular,

    /// [G75] Must be called before creating the first arc.
    ArcInit,

    /// [LP] Loads the polarity object transformation parameter.
    LoadPolarity,

    /// [LM] Loads the mirror object transformation parameter.
    LoadMirroring,

    /// [LR] Loads the rotation object transformation parameter.
    LoadRotation,

    /// [LS] Loads the scale object transformation parameter.
    LoadScaling,

    /// [G36] Starts a region statement which creates a region by
    /// defining its contours.
    StartRegion,

    /// [G37] Ends the region statement.
    EndRegion,

    /// [AB] Opens a block aperture statement and assigns its aperture
    /// number or closes a block aperture statement.
    ApertureBlock,

    /// [SR] Open or closes a step and repeat statement.
    StepAndRepeat,

    /// [TF] Set a file attribute.
    AttributeOnFile,

    /// [TA] Add an aperture attribute to the dictionary or modify it.
    AttributeOnAperture,

    /// [TO] Add an object attribute to the dictionary or modify it.
    AttributeOnObject,

    /// [TD] Delete one or all attributes in the dictionary.
    AttributeDelete,

    /// [M02] End of file.
    EndOfFile,
}

pub use Command::*;

/// [Comment] A human readable comment, does not affect the image.
pub const G04: Command = Comment;

/// [Mode] Sets the unit to mm or inch.
pub const MO: Command = Mode;

/// [FormatSpecification] Sets the coordinate format, e.g. the number of decimals.
pub const FS: Command = FormatSpecification;

/// [ApertureDefine] Defines a template-based aperture, assigns a D code to it.
pub const AD: Command = ApertureDefine;

/// [ApertureMacro] Defines a macro aperture template.
pub const AM: Command = ApertureMacro;

/// [SetCurrentAperture] (Dnn for nn≥10) Sets the current aperture to D code nn.
pub const D: Command = SetCurrentAperture;

/// [Plot] Outside a region statement [D01] creates a draw or arc
/// object with the current aperture. Inside it adds a draw/arc
/// segment to the contour under construction. The current
/// point is moved to draw/arc end point after the creation of
/// the draw/arc.
pub const D01: Command = Plot;

/// [Move] Moves the current point to the coordinate in the
/// command. It does not create an object.
pub const D02: Command = Move;

/// [Flash] Creates a flash object with the current aperture. The
/// current point is moved to the flash point.
pub const D03: Command = Flash;

/// [SetLinear] Sets linear/circular mode to linear.
pub const G01: Command = SetLinear;

/// [SetCWCircular] Sets linear/circular mode to clockwise circular.
pub const G02: Command = SetCWCircular;

/// [SetCCWCircular] Sets linear/circular mode to counterclockwise circular.
pub const G03: Command = SetCCWCircular;

/// [ArcInit] Must be called before creating the first arc.
pub const G75: Command = ArcInit;

/// [LoadPolarity] Loads the polarity object transformation parameter.
pub const LP: Command = LoadPolarity;

/// [LoadMirroring] Loads the mirror object transformation parameter.
pub const LM: Command = LoadMirroring;

/// [LoadRotation] Loads the rotation object transformation parameter.
pub const LR: Command = LoadRotation;

/// [LoadScaling] Loads the scale object transformation parameter.
pub const LS: Command = LoadScaling;

/// [StartRegion] Starts a region statement which creates a region by
/// defining its contours.
pub const G36: Command = StartRegion;

/// [EndRegion] Ends the region statement.
pub const G37: Command = EndRegion;

/// [ApertureBlock] Opens a block aperture statement and assigns its aperture
/// number or closes a block aperture statement.
pub const AB: Command = ApertureBlock;

/// [StepAndRepeat] Open or closes a step and repeat statement.
pub const SR: Command = StepAndRepeat;

/// [AttributeOnFile] Set a file attribute.
pub const TF: Command = AttributeOnFile;

/// [AttributeOnAperture] Add an aperture attribute to the dictionary or modify it.
pub const TA: Command = AttributeOnAperture;

/// [AttributeOnObject] Add an object attribute to the dictionary or modify it.
pub const TO: Command = AttributeOnObject;

/// [AttributeDelete] Delete one or all attributes in the dictionary.
pub const TD: Command = AttributeDelete;

/// [EndOfFile] End of file.
pub const M02: Command = EndOfFile;
