#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fonts{
	RobotoBlack,
	OpenSansBold,
	RobotoMedium,
	OpenSansExtraBold,
	RobotoCondensedItalic,
	OpenSansExtraBoldItalic,
	RobotoThinItalic,
	RobotoThin,
	DejaVuSans,
	RobotoBold,
	OpenSansRegular,
	OpenSansSemibold,
	OpenSansLight,
	RobotoRegular,
	RobotoCondensedRegular,
	RobotoCondensedBold,
	RobotoCondensedLightItalic,
	OpenSansItalic,
	OpenSansBoldItalic,
	OpenSansSemiboldItalic,
	RobotoItalic,
	OpenSansLightItalic,
	RobotoCondensedBoldItalic,
	RobotoMediumItalic,
	RobotoBoldItalic,
	RobotoCondensedLight,
	RobotoLight,
	RobotoLightItalic,
	RobotoBlackItalic,
}
impl Fonts{
    pub fn get_ttf(self)->&'static [u8]{
        match self{
            	Fonts::RobotoBlack=>{ let ttf1 = include_bytes!( "fonts/Roboto-Black.ttf");
	return ttf1;},
	Fonts::OpenSansBold=>{ let ttf1 = include_bytes!( "fonts/OpenSans-Bold.ttf");
	return ttf1;},
	Fonts::RobotoMedium=>{ let ttf1 = include_bytes!( "fonts/Roboto-Medium.ttf");
	return ttf1;},
	Fonts::OpenSansExtraBold=>{ let ttf1 = include_bytes!( "fonts/OpenSans-ExtraBold.ttf");
	return ttf1;},
	Fonts::RobotoCondensedItalic=>{ let ttf1 = include_bytes!( "fonts/RobotoCondensed-Italic.ttf");
	return ttf1;},
	Fonts::OpenSansExtraBoldItalic=>{ let ttf1 = include_bytes!( "fonts/OpenSans-ExtraBoldItalic.ttf");
	return ttf1;},
	Fonts::RobotoThinItalic=>{ let ttf1 = include_bytes!( "fonts/Roboto-ThinItalic.ttf");
	return ttf1;},
	Fonts::RobotoThin=>{ let ttf1 = include_bytes!( "fonts/Roboto-Thin.ttf");
	return ttf1;},
	Fonts::DejaVuSans=>{ let ttf1 = include_bytes!( "fonts/DejaVuSans.ttf");
	return ttf1;},
	Fonts::RobotoBold=>{ let ttf1 = include_bytes!( "fonts/Roboto-Bold.ttf");
	return ttf1;},
	Fonts::OpenSansRegular=>{ let ttf1 = include_bytes!( "fonts/OpenSans-Regular.ttf");
	return ttf1;},
	Fonts::OpenSansSemibold=>{ let ttf1 = include_bytes!( "fonts/OpenSans-Semibold.ttf");
	return ttf1;},
	Fonts::OpenSansLight=>{ let ttf1 = include_bytes!( "fonts/OpenSans-Light.ttf");
	return ttf1;},
	Fonts::RobotoRegular=>{ let ttf1 = include_bytes!( "fonts/Roboto-Regular.ttf");
	return ttf1;},
	Fonts::RobotoCondensedRegular=>{ let ttf1 = include_bytes!( "fonts/RobotoCondensed-Regular.ttf");
	return ttf1;},
	Fonts::RobotoCondensedBold=>{ let ttf1 = include_bytes!( "fonts/RobotoCondensed-Bold.ttf");
	return ttf1;},
	Fonts::RobotoCondensedLightItalic=>{ let ttf1 = include_bytes!( "fonts/RobotoCondensed-LightItalic.ttf");
	return ttf1;},
	Fonts::OpenSansItalic=>{ let ttf1 = include_bytes!( "fonts/OpenSans-Italic.ttf");
	return ttf1;},
	Fonts::OpenSansBoldItalic=>{ let ttf1 = include_bytes!( "fonts/OpenSans-BoldItalic.ttf");
	return ttf1;},
	Fonts::OpenSansSemiboldItalic=>{ let ttf1 = include_bytes!( "fonts/OpenSans-SemiboldItalic.ttf");
	return ttf1;},
	Fonts::RobotoItalic=>{ let ttf1 = include_bytes!( "fonts/Roboto-Italic.ttf");
	return ttf1;},
	Fonts::OpenSansLightItalic=>{ let ttf1 = include_bytes!( "fonts/OpenSans-LightItalic.ttf");
	return ttf1;},
	Fonts::RobotoCondensedBoldItalic=>{ let ttf1 = include_bytes!( "fonts/RobotoCondensed-BoldItalic.ttf");
	return ttf1;},
	Fonts::RobotoMediumItalic=>{ let ttf1 = include_bytes!( "fonts/Roboto-MediumItalic.ttf");
	return ttf1;},
	Fonts::RobotoBoldItalic=>{ let ttf1 = include_bytes!( "fonts/Roboto-BoldItalic.ttf");
	return ttf1;},
	Fonts::RobotoCondensedLight=>{ let ttf1 = include_bytes!( "fonts/RobotoCondensed-Light.ttf");
	return ttf1;},
	Fonts::RobotoLight=>{ let ttf1 = include_bytes!( "fonts/Roboto-Light.ttf");
	return ttf1;},
	Fonts::RobotoLightItalic=>{ let ttf1 = include_bytes!( "fonts/Roboto-LightItalic.ttf");
	return ttf1;},
	Fonts::RobotoBlackItalic=>{ let ttf1 = include_bytes!( "fonts/Roboto-BlackItalic.ttf");
	return ttf1;},
        }
    }
}
