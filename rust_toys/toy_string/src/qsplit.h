//
// Created by zhyi on 2020-03-08.
//

#ifndef TOY_STRING_QSPLIT_H
#define TOY_STRING_QSPLIT_H

#ifdef __cplusplus
extern "C"{
#endif

//std::vector <std::string>
//long_text_split( const std::string &strs, const std::string &delims, const std::string &second_delims,
//                 size_t max_sub_wcount );

std::vector <std::string>
long_text_split_ffi( const char* strs, const char* delims, const char* second_delims,
                 size_t max_sub_wcount );

#ifdef __cplusplus

}
#endif

#endif //TOY_STRING_QSPLIT_H
