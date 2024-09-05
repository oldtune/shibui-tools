use anyhow::bail;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};

use crate::scraper::scraper::{
    Class, ClassDefinition, Region, WordClass, WordDefinition, WordPronounce,
};

pub struct WordPage<'a> {
    pub content: &'a Html,
}

impl<'a> WordPage<'a> {
    //There is a possibility that the word does not exist.
    pub fn new(html: &'a Html) -> anyhow::Result<Self> {
        Ok(Self { content: html })
    }

    fn word_class_sections(&'a self) -> Vec<WordClassSection<'a>> {
        let selector = Selector::parse(".pr.entry-body__el").unwrap();
        self.content
            .select(&selector)
            .map(|ele| WordClassSection::new(ele, &self))
            .collect()
    }

    pub fn get_word_definition(&self) -> anyhow::Result<WordDefinition> {
        let word_classes = self.word_class_sections();
        if word_classes.len() == 0 {
            bail!("Word not found");
        }

        let word = word_classes.first().unwrap().get_current_word();
        let word_definition = WordDefinition {
            word,
            classes: word_classes
                .iter()
                .map(|class| class.get_word_class_definition())
                .collect(),
        };

        Ok(word_definition)
    }
}

pub struct WordClassSection<'a> {
    //the word class ele
    pub inner_html_ele: ElementRef<'a>,
    word_page: &'a WordPage<'a>,
}

impl<'a> WordClassSection<'a> {
    pub fn new(inner_html_ele: ElementRef<'a>, word_page: &'a WordPage) -> Self {
        Self {
            word_page,
            inner_html_ele,
        }
    }

    pub fn get_current_word(&self) -> String {
        let header = self.header();
        header.get_word()
    }

    pub fn get_word_class_definition(&self) -> WordClass {
        let header = self.header();
        // let definitions = self.definitions();
        WordClass {
            class: header.get_class(),
            pronounces: vec![],
            // pronounces: header.get_pronounces(),
            // definitions: definitions.iter().map(|x| x.get_definition()).collect(),
            definitions: vec![],
        }
    }

    pub fn definitions(&'a self) -> Vec<ClassDefinitionSection<'a>> {
        todo!()
    }

    pub fn header(&'a self) -> WordClassHeaderSection<'a> {
        let selector = Selector::parse(".pos-header.dpos-h").unwrap();
        let header: WordClassHeaderSection<'a> = self
            .inner_html_ele
            .select(&selector)
            .map(|ele| WordClassHeaderSection::new(ele, self))
            .next()
            .unwrap();

        header
    }
}

pub struct WordClassHeaderSection<'a> {
    pub inner_html_ele: ElementRef<'a>,
    word_class: &'a WordClassSection<'a>,
}

impl<'a> WordClassHeaderSection<'a> {
    pub fn new(inner_html_ele: ElementRef<'a>, word_class: &'a WordClassSection) -> Self {
        Self {
            inner_html_ele,
            word_class,
        }
    }

    pub fn get_word(&self) -> String {
        let selector = Selector::parse(".hw.dhw").unwrap();
        let header_ele = self.inner_html_ele.select(&selector).next().unwrap();
        let text = header_ele.text().next();
        return text.unwrap().to_owned();
    }

    pub fn get_class(&self) -> Class {
        let selector = Selector::parse(".pos.dpos").unwrap();
        let class_ele = self.inner_html_ele.select(&selector).next().unwrap();
        let text = class_ele.text().next().unwrap();
        text.into()
    }

    pub fn get_pronounces(&self) -> Vec<WordPronounce> {
        let result = vec![
            WordPronounce {
                link: self.get_uk_sound_link(),
                ipa: self.get_uk_ipa(),
                region: Region::UK,
            },
            WordPronounce {
                ipa: self.get_us_ipa(),
                link: self.get_us_sound_link(),
                region: Region::US,
            },
        ];

        result
    }

    fn get_uk_sound_link(&self) -> String {
        todo!()
    }

    fn get_us_sound_link(&self) -> String {
        todo!()
    }

    fn get_uk_ipa(&self) -> String {
        todo!()
    }

    fn get_us_ipa(&self) -> String {
        todo!()
    }
}

pub struct ClassDefinitionSection<'a> {
    pub inner_html_ele: ElementRef<'a>,
    word_class: &'a WordClassSection<'a>,
}

impl<'a> ClassDefinitionSection<'a> {
    pub fn new(inner_html_ele: ElementRef<'a>, word_class: &'a WordClassSection<'a>) -> Self {
        Self {
            inner_html_ele,
            word_class,
        }
    }

    pub fn get_definition(&self) -> ClassDefinition {
        todo!()
    }
}
