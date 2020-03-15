//
// Created by zhyi on 2020-03-08.
//

#ifndef TOY_STRING_QSPLIT_TS_H
#define TOY_STRING_QSPLIT_TS_H

class LongText
{
    void *internal;
public:
    LongText();
    ~LongText();
    size_t bytes_count() const;
    size_t chars_count() const;
    void set_text(const std::string& text);
    std::vector<std::string> split(const std::string& delims,const std::string& second_delims,size_t max_sub_wcount) const;

public:
    std::string text;
    size_t size;
};

std::vector<std::string> long_text_split(const std::string& strs,const std::string& delims,const std::string& second_delims,size_t max_sub_wcount);

#endif //TOY_STRING_QSPLIT_TS_H
